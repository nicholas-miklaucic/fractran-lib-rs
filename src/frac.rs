//! A representation of a fraction used in Fractran program execution.

use super::primebasis::{Divides, PrimeBasis};
use std::fmt;
use std::ops::{Div, Mul};

/// Wrapper trait for the various things that numbers in Fractran programs need
/// to do. `PrimeBasis` satisfies this, as does `u64`.
pub trait FractranNat:
    Into<u64>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Divides
    + Clone
    + std::fmt::Debug
    + Sized
{
}
impl<T> FractranNat for T where
    T: Into<u64>
        + Mul<Self, Output = Self>
        + Div<Self, Output = Self>
        + Divides
        + Clone
        + std::fmt::Debug
        + Sized
{
}

/// A fraction in Fractran, with a nonzero numerator and denominator.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fraction<T: FractranNat> {
    /// The fraction numerator: must be nonzero.
    num: T,
    /// The fraction denominator: must be nonzero.
    denom: T,
}

/// The result of executing an instruction step in Fractran. `Changed` means
/// that the input did divide the fraction, and `Unchanged` means it did not.
/// Note that the result of a step can be `Changed` and still equal to the
/// original state, if the fraction used was 1/1.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum StepResult<T: FractranNat> {
    /// The same state as before, without any multiplication.
    Unchanged(T),
    /// The product of the former state and the fraction at this step.
    Changed(T),
}

impl<T: FractranNat> Fraction<T> {
    /// Creates a new `Fraction` with the given numerator and denominator,
    /// panicking if either input is zero.
    pub fn new(num: T, denom: T) -> Fraction<T> {
        if num.clone().into() == 0_u64 || denom.clone().into() == 0_u64 {
            panic!("Cannot have fraction with zero on either side!");
        } else {
            Fraction { num, denom }
        }
    }
    /// Computes the only operation Fractran has: for this fraction `f` and some
    /// input `n`, returns `StepResult::Changed(nf)` if `nf` is integral and
    /// `StepResult::Unchanged(n)` otherwise. Note that, for example, 1/1
    /// doesn't change the actual state, but it will still return `Changed`
    /// because the multiplication was performed.
    pub(crate) fn exec(&self, input: T) -> StepResult<T> {
        let new_num = input.clone() * self.clone().num;
        if self.denom.divides(&new_num) {
            StepResult::Changed(new_num / self.clone().denom)
        } else {
            StepResult::Unchanged(input)
        }
    }
}

impl<T: FractranNat + fmt::Display> fmt::Display for Fraction<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} / {}", self.num, self.denom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        assert_eq!(
            Fraction::new(1_u64, 2_u64).exec(2_u64),
            StepResult::Changed(1_u64)
        );
        assert_eq!(
            Fraction::new(1_u64, 2_u64).exec(1_u64),
            StepResult::Unchanged(1_u64)
        );
        assert_eq!(
            Fraction::new(6_u64, 7_u64).exec(28_u64),
            StepResult::Changed(24_u64)
        );
    }
}
