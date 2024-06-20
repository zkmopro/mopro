use halo2_proofs::halo2curves::ff::PrimeField as FieldExt;
use halo2_proofs::halo2curves::ff::PrimeField;
use halo2_proofs::plonk::Expression;

/// Decodes a field element from its byte representation in little endian order
pub mod from_bytes {
    use super::{Expr, Expression, PrimeField};

    pub fn expr<F: PrimeField, E: Expr<F>>(bytes: &[E]) -> Expression<F> {
        let mut value = 0.expr();
        let mut multiplier = F::ONE;
        for byte in bytes.iter() {
            value = value + byte.expr() * multiplier;
            multiplier *= F::from(256);
        }
        value
    }

    pub fn value<F: PrimeField>(bytes: &[u8]) -> F {
        let mut value = F::ZERO;
        let mut multiplier = F::ONE;
        let two_pow_64 = F::from_u128(1u128 << 64);
        let two_pow_128 = two_pow_64 * two_pow_64;
        for u128_chunk in bytes.chunks(u128::BITS as usize / u8::BITS as usize) {
            let mut buffer = [0; 16];
            buffer[..u128_chunk.len()].copy_from_slice(u128_chunk);
            value += F::from_u128(u128::from_le_bytes(buffer)) * multiplier;
            multiplier *= two_pow_128;
        }
        value
    }
}

/// Returns the sum of the passed in cells
pub mod sum {
    use super::{Expr, Expression, FieldExt};

    /// Returns an expression for the sum of the list of expressions.
    pub fn expr<F: FieldExt, E: Expr<F>, I: IntoIterator<Item = E>>(inputs: I) -> Expression<F> {
        inputs
            .into_iter()
            .fold(0.expr(), |acc, input| acc + input.expr())
    }
}

/// Returns `1` when `expr[0] && expr[1] && ... == 1`, and returns `0`
/// otherwise. Inputs need to be boolean
pub mod and {
    use super::{Expr, Expression, FieldExt};

    /// Returns an expression that evaluates to 1 only if all the expressions in
    /// the given list are 1, else returns 0.
    pub fn expr<F: FieldExt, E: Expr<F>, I: IntoIterator<Item = E>>(inputs: I) -> Expression<F> {
        inputs
            .into_iter()
            .fold(1.expr(), |acc, input| acc * input.expr())
    }
}

/// Returns `1` when `expr[0] || expr[1] || ... == 1`, and returns `0`
/// otherwise. Inputs need to be boolean
pub mod or {
    use super::{and, not};
    use super::{Expr, Expression, FieldExt};

    /// Returns an expression that evaluates to 1 if any expression in the given
    /// list is 1. Returns 0 if all the expressions were 0.
    pub fn expr<F: FieldExt, E: Expr<F>, I: IntoIterator<Item = E>>(inputs: I) -> Expression<F> {
        not::expr(and::expr(inputs.into_iter().map(not::expr)))
    }
}

/// Returns `1` when `b == 0`, and returns `0` otherwise.
/// `b` needs to be boolean
pub mod not {
    use super::{Expr, Expression, FieldExt};

    /// Returns an expression that represents the NOT of the given expression.
    pub fn expr<F: FieldExt, E: Expr<F>>(b: E) -> Expression<F> {
        1.expr() - b.expr()
    }
}

/// Returns `when_true` when `selector == 1`, and returns `when_false` when
/// `selector == 0`. `selector` needs to be boolean.
pub mod select {
    use super::{Expr, Expression, FieldExt};

    /// Returns the `when_true` expression when the selector is true, else
    /// returns the `when_false` expression.
    pub fn expr<F: FieldExt>(
        selector: Expression<F>,
        when_true: Expression<F>,
        when_false: Expression<F>,
    ) -> Expression<F> {
        selector.clone() * when_true + (1.expr() - selector) * when_false
    }
}

/// Trait that implements functionality to get a constant expression from
/// commonly used types.
pub trait Expr<F: FieldExt> {
    /// Returns an expression for the type.
    fn expr(&self) -> Expression<F>;
}

/// Implementation trait `Expr` for type able to be casted to u64
#[macro_export]
macro_rules! impl_expr {
    ($type:ty) => {
        impl<F: FieldExt> Expr<F> for $type {
            #[inline]
            fn expr(&self) -> Expression<F> {
                Expression::Constant(F::from(*self as u64))
            }
        }
    };
    ($type:ty, $method:path) => {
        impl<F: FieldExt> Expr<F> for $type {
            #[inline]
            fn expr(&self) -> Expression<F> {
                Expression::Constant(F::from($method(self) as u64))
            }
        }
    };
}

impl_expr!(bool);
impl_expr!(u8);
impl_expr!(u64);
impl_expr!(usize);

impl<F: FieldExt> Expr<F> for Expression<F> {
    #[inline]
    fn expr(&self) -> Expression<F> {
        self.clone()
    }
}

impl<F: FieldExt> Expr<F> for &Expression<F> {
    #[inline]
    fn expr(&self) -> Expression<F> {
        (*self).clone()
    }
}

impl<F: FieldExt> Expr<F> for i32 {
    #[inline]
    fn expr(&self) -> Expression<F> {
        Expression::Constant(
            F::from(self.unsigned_abs() as u64) * if self.is_negative() { -F::ONE } else { F::ONE },
        )
    }
}
