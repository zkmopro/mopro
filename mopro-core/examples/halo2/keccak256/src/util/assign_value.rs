use crate::util::prime_field::ScalarField;
use crate::util::Halo2AssignedCell;
use halo2_proofs::arithmetic::Field;
use halo2_proofs::circuit::{Cell, Region, Value};
use halo2_proofs::plonk::{Advice, Assigned, Column, Fixed};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct AssignedPrimitive<T: Into<u64> + Copy, F: ScalarField> {
    pub value: Value<T>,

    pub cell: Cell,
    _row_offset: usize,
    _marker: PhantomData<F>,
}

#[derive(Clone, Debug, Copy)]
pub struct AssignedValue<F: ScalarField> {
    pub cell: Cell,
    pub value: Value<F>,
    pub row_offset: usize,
    pub(crate) _marker: PhantomData<F>,
}

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

impl<'a, F: ScalarField> AssignedValue<F> {
    pub fn row(&self) -> usize {
        {
            self.row_offset
        }
    }

    pub fn cell(&self) -> Cell {
        self.cell
    }

    pub fn value(&self) -> Value<&F> {
        {
            self.value.as_ref()
        }
    }

    pub fn copy_advice(
        &self,
        region: &mut Region<'_, F>,
        column: Column<Advice>,
        offset: usize,
    ) -> Cell {
        let cell = region
            .assign_advice(|| "", column, offset, || self.value)
            .expect("assign copy advice should not fail")
            .cell();
        region
            .constrain_equal(cell, self.cell())
            .expect("constrain equal should not fail");

        cell
    }
}
