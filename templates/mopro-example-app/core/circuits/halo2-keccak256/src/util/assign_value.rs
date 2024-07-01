use crate::util::Halo2AssignedCell;
use halo2_proofs::arithmetic::Field;
use halo2_proofs::circuit::{Cell, Region, Value};
use halo2_proofs::plonk::{Advice, Assigned, Column, Fixed};

/// Assign advice to physical region.
#[inline(always)]
pub fn raw_assign_advice<'v, F: Field>(
    region: &mut Region<F>,
    column: Column<Advice>,
    offset: usize,
    value: Value<impl Into<Assigned<F>>>,
) -> Halo2AssignedCell<'v, F> {
    let value = value.map(|a| Into::<Assigned<F>>::into(a));
    region
        .assign_advice(
            || format!("assign advice {column:?} offset {offset}"),
            column,
            offset,
            || value,
        )
        .unwrap()
}

#[inline(always)]
pub fn raw_assign_fixed<F: Field>(
    region: &mut Region<F>,
    column: Column<Fixed>,
    offset: usize,
    value: F,
) -> Cell {
    region
        .assign_fixed(
            || format!("assign fixed {column:?} offset {offset}"),
            column,
            offset,
            || Value::known(value),
        )
        .unwrap()
        .cell()
}
