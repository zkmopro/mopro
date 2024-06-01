use std::marker::PhantomData;
use std::ops::Mul;

use halo2_proofs::{circuit::*, plonk::*, poly::Rotation};
use halo2_proofs::arithmetic::Field;

#[derive(Debug, Clone)]

// Defines the configuration of all the columns, and all of the column definitions
// Will be incrementally populated and passed around
pub struct Multiplier2Config {
    pub col_a: Column<Advice>,
    pub col_b: Column<Advice>,
    pub col_c: Column<Advice>,
    pub selector: Selector,
    pub instance_a: Column<Instance>,
}

#[derive(Debug, Clone)]
struct Multiplier2Chip<F: Field> {
    config: Multiplier2Config,
    _marker: PhantomData<F>,
    // In rust, when you have a struct that is generic over a type parameter (here F),
    // but the type parameter is not referenced in a field of the struct,
    // you have to use PhantomData to virtually reference the type parameter,
    // so that the compiler can track it.  Otherwise it would give an error. - Jason
}

impl<F: Field> Multiplier2Chip<F> {
    // Default constructor
    pub fn construct(config: Multiplier2Config) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    // Configure will set what type of columns things are, enable equality, create gates, and return a config with all the gates
    pub fn configure(meta: &mut ConstraintSystem<F>) -> Multiplier2Config {
        let col_a = meta.advice_column();
        let col_b = meta.advice_column();
        let col_c = meta.advice_column();
        let selector = meta.selector();
        let instance_a = meta.instance_column();

        // enable_equality has some cost, so we only want to define it on rows where we need copy constraints
        meta.enable_equality(col_a);
        meta.enable_equality(instance_a);

        // Defining a create_gate here applies it over every single column in the circuit.
        // We will use the selector column to decide when to turn this gate on and off, since we probably don't want it on every row
        meta.create_gate("mul", |meta| {
            //
            // col_a | col_b | col_c | selector
            //   a      b        c       s
            //
            let s = meta.query_selector(selector);
            let a = meta.query_advice(col_a, Rotation::cur());
            let b = meta.query_advice(col_b, Rotation::cur());
            let c = meta.query_advice(col_c, Rotation::cur());
            vec![s * (a * b - c)]
        });

        Multiplier2Config {
            col_a,
            col_b,
            col_c,
            selector,
            instance_a,
        }
    }

    pub fn assign(
        &self,
        a: Value<F>,
        b: Value<F>,
        mut layouter: impl Layouter<F>,
    ) -> Result<AssignedCell<F, F>, Error> {
        layouter.assign_region(
            || "assign mul row",
            |mut region| {
                self.config.selector.enable(&mut region, 0)?;


                let a_cell = region.assign_advice(
                    || "a",
                    self.config.col_a,
                    0,
                    || a,
                )?;

                let b_cell = region.assign_advice(
                    || "b",
                    self.config.col_b,
                    0,
                    || b,
                )?;

                region.assign_advice(
                    || "a * b = c",
                    self.config.col_c,
                    0,
                    || a.mul(&b),
                )?;

                Ok(a_cell)
            },
        )
    }

}

#[derive(Default)]
pub struct Multiplier2Circuit<F> {
    pub a: Value<F>,
    pub b: Value<F>,
}

// Our circuit will instantiate an instance based on the interface defined on the chip and floorplanner (layouter)
// There isn't a clear reason this and the chip aren't the same thing, except for better abstractions for complex circuits
impl<F: Field> Circuit<F> for Multiplier2Circuit<F> {
    type Config = Multiplier2Config;
    type FloorPlanner = SimpleFloorPlanner;

    // Circuit without witnesses, called only during key generation
    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    // Has the arrangement of columns. Called only during keygen, and will just call chip config most of the time
    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        Multiplier2Chip::configure(meta)
    }

    // Take the output of configure and floorplanner type to make the actual circuit
    // Called both at key generation time, and proving time with a specific witness
    // Will call all of the copy constraints
    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = Multiplier2Chip::construct(config);

        let a_cell = chip.assign(self.a, self.b, layouter.namespace(|| "assign multiply"))?;

        layouter.namespace(|| "check output").constrain_instance(
            a_cell.cell(),
            chip.config.instance_a,
            0,
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use halo2_proofs::circuit::Value;
    use halo2_proofs::dev::MockProver;
    use halo2_proofs::halo2curves::pasta::Fp;

    use super::Multiplier2Circuit;

    #[test]
    fn circuit_example() {
        let k = 4;

        let a = Fp::from(100);
        let b = Fp::from(10);

        let out = a.mul(&b);

        let public_input = vec![vec![a]];

        let circuit = Multiplier2Circuit::<Fp> {
            a: Value::known(a),
            b: Value::known(b),
        };

        // This prover is faster and 'fake', but is mostly a devtool for debugging
        let prover = MockProver::run(k, &circuit, public_input).unwrap();
        // This function will pretty-print on errors
        prover.assert_satisfied();
    }

    #[cfg(feature = "dev-graph")]
    #[test]
    fn plot_circuit() {
        use plotters::prelude::*;

        let root = BitMapBackend::new("fib-1-layout.png", (1024, 3096)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.titled("Fib 1 Layout", ("sans-serif", 60)).unwrap();

        let circuit = Multiplier2Circuit::<Fp> {
            a: Value::unknown(),
            b: Value::unknown(),
        };

        halo2_proofs::dev::CircuitLayout::default()
            .render(4, &circuit, &root)
            .unwrap();
    }
}
