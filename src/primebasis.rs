//! Defines a type that behaves like a natural number but is stored in
//! factorized form for computational efficiency when executing Fractran
//! programs.

use std::convert::{Into, TryFrom};
use std::format;
use std::ops::{Div, Mul, Rem};

use itertools::EitherOrBoth;
use itertools::Itertools;
use thiserror::Error;

use super::PRIMES;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Register overflow: input {0} has prime factor larger than {}",
            PRIMES.last().unwrap())]
    RegisterOverflow(u64),

    #[error("Zero is meaningless in FRACTRAN programs, cannot be stored")]
    NumIsZero,
}

/// Trait that expresses the ability to determine if a number divides another
/// number. Can be thought of as a superset of `std::ops::Rem`, because it only
/// requires knowing whether the remainder is 0 or not.
pub trait Divides {
    /// Returns `true` if `rhs` is a multiple of `self` and `false` otherwise.
    fn divides(&self, rhs: &Self) -> bool;
}

// implement this for specifically u64 but anything else that happens to fit
impl<T> Divides for T
where
    T: Rem<Self, Output = Self> + Into<u64> + Eq + Copy,
{
    fn divides(&self, rhs: &Self) -> bool {
        (*rhs % *self).into() == 0
    }
}

/// A natural number, represented as a vector of exponents in the prime
/// factorization [a, b, c, ...] = 2^a * 3^b * 5^c * ...
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrimeBasis {
    /// The vector of exponents. Can be at most `MAX_REGS`, but is not
    /// guaranteed to be that long: any missing exponents are presumed to be 0.
    pub exps: Vec<u32>,
}

impl PrimeBasis {
    /// Attempts to create the prime basis representation of the given natural
    /// number. Returns `RegisterOverflow` if the number cannot be factored
    /// using the available space, and `NumIsZero` if the number given is zero,
    /// which has no prime factors.
    pub fn try_new(num: u64) -> Result<PrimeBasis, Error> {
        if num == 0 {
            return Err(Error::NumIsZero);
        }
        let mut exps = vec![];
        let mut exp;
        let mut curr = num;
        for prime in &*PRIMES {
            if curr == 1 {
                // we're done, return the value
                return Ok(PrimeBasis { exps });
            }

            exp = 0;
            while curr % prime == 0 {
                curr /= prime;
                exp += 1;
            }
            exps.push(exp);
        }
        // if we reach here, didn't fully factor
        Err(Error::RegisterOverflow(num))
    }

    /// Returns the number corresponding to this prime basis.
    pub fn value(&self) -> u64 {
        self.exps
            .iter()
            .zip(&*PRIMES)
            .fold(1, |acc, (&exp, p)| acc * p.pow(exp))
    }
}

impl std::fmt::Display for PrimeBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let godel_str = self
            .exps
            .iter()
            .zip(&*PRIMES)
            .map(|(&exp, p)| {
                if exp != 0 {
                    format!("{}^{}", p, exp)
                } else {
                    String::new()
                }
            })
            .filter(|s| s != "")
            .join(" ✕ ");

        if godel_str == "" {
            write!(f, "PrimeBasis(1)")
        } else {
            write!(f, "PrimeBasis({})", godel_str)
        }
    }
}

impl Mul for PrimeBasis {
    type Output = PrimeBasis;

    /// Returns the `PrimeBasis` representing the product of the numbers that
    /// the input bases represent.
    fn mul(self, rhs: Self) -> Self::Output {
        // it's pretty interesting how multiplication in the normal sense
        // becomes addition in the prime basis sense
        PrimeBasis {
            exps: self
                .exps
                .into_iter()
                .zip_longest(rhs.exps)
                .map(|pair| pair.reduce(|a, b| a + b))
                .collect(),
        }
    }
}

impl Div for PrimeBasis {
    type Output = PrimeBasis;

    /// Returns the `PrimeBasis` representing the quotient of the numbers that
    /// the input bases represent. Panics if the output would not be a natural
    /// number.
    fn div(self, rhs: Self) -> Self::Output {
        if !rhs.divides(&self) {
            panic!("Can't divide {} by {}", self, rhs);
        } else {
            PrimeBasis {
                exps: self
                    .exps
                    .into_iter()
                    .zip_longest(rhs.exps)
                    .map(|pair| pair.reduce(|a, b| a - b))
                    .collect(),
            }
        }
    }
}

impl Divides for PrimeBasis {
    /// Checks if `rhs` is a multiple of `self`.
    fn divides(&self, rhs: &Self) -> bool {
        self.exps.iter().zip_longest(&rhs.exps).all(|pair| {
            match pair {
                // if no value on right side, left must be 0
                EitherOrBoth::Left(a) => *a == 0,
                // if no value on left side, right can be anything
                EitherOrBoth::Right(_) => true,
                // else, left must be <= right
                EitherOrBoth::Both(a, b) => a <= b,
            }
        })
    }
}

impl TryFrom<u64> for PrimeBasis {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl Into<u64> for PrimeBasis {
    /// Returns the natural number that is represented by this prime basis.
    fn into(self) -> u64 {
        self.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Constructor that panics instead of returning a result
    fn new(num: u64) -> PrimeBasis {
        PrimeBasis::try_new(num).unwrap()
    }

    #[test]
    fn test_basis_nums() {
        // 200 = 8 * 25 = 2^3 * 5^2
        assert_eq!(new(200).exps, vec![3, 0, 2]);
        // 1 has no values
        assert_eq!(new(1).exps, vec![]);
        // 0 has error
        assert_eq!(PrimeBasis::try_new(0), Err(Error::NumIsZero));
    }

    #[test]
    fn test_tryfrom_u64() {
        let nums: Vec<u64> = vec![1, 2, 3, 5, 10, 20, 60, 2520, 70000];
        for num in nums {
            let converted: u64 = PrimeBasis::try_from(num).unwrap().into();
            assert_eq!(num, converted);
        }
    }

    #[test]
    fn test_into_u64() {
        let nums: Vec<u64> = vec![1, 2, 3, 5, 10, 20, 60, 2520, 70000];
        for num in nums {
            let converted: u64 = new(num).into();
            assert_eq!(num, converted);
        }
    }

    #[test]
    fn test_mul() {
        let nums1: Vec<u64> = vec![1, 2, 5, 8, 100, 256, 2520];
        let nums2: Vec<u64> = vec![1, 5, 7, 11, 102, 353, 1000];
        for num1 in &nums1 {
            for num2 in &nums2 {
                let pb1 = PrimeBasis::try_new(*num1).unwrap();
                let pb2 = PrimeBasis::try_new(*num2).unwrap();
                let ans: u64 = (pb1 * pb2).into();
                assert_eq!(ans, num1 * num2);
            }
        }
    }

    #[test]
    fn test_divides() {
        let help_div = |a, b| {
            let pb1 = new(a);
            let pb2 = new(b);
            pb1.divides(&pb2)
        };
        assert_eq!(help_div(7, 28), true);
        assert_eq!(help_div(32, 128), true);
        assert_eq!(help_div(40, 40), true);
        assert_eq!(help_div(1, 28), true);
        assert_eq!(help_div(1, 1), true);
        assert_eq!(help_div(70, 7), false);
        assert_eq!(help_div(2, 7), false);
        assert_eq!(help_div(100, 250), false);
    }
}
