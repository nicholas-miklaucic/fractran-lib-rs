//! Provides a struct to represent program in Fractran, represented as a list of
//! fractions.

use super::frac::{Fraction, FractranNat, StepResult};
use std::iter::Iterator;

/// A program in Fractran: a list of fractions. Execution proceeds by
/// multiplying the input number by each fraction in turn, overwriting the
/// current number only if the product is an integer. Execution ends when the
/// state stops changing.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Program<T: FractranNat> {
    /// The list of fractions that comprises the program.
    fracs: Vec<Fraction<T>>,
}

impl<T: FractranNat> Program<T> {
    /// Makes a new `Program` with the given nonempty list of fractions.
    pub fn new(fracs: Vec<Fraction<T>>) -> Program<T> {
        Program{
            fracs
        }
    }
}

/// An iterator that holds the state of a program as it runs and, each time
/// `next()` is called, continues to evaluate the program.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Evaluator<T: FractranNat> {
    /// The program being run as a list of fractions.
    program: Vec<Fraction<T>>,

    /// The current state of the program.
    curr_state: T,

    /// Whether this program is over.
    finished: bool,
}

impl<T: FractranNat> Iterator for Evaluator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            for frac in self.program.clone() {
                if let StepResult::Changed(new_state) = frac.exec(self.curr_state.clone()) {
                    self.curr_state = new_state;
                    return Some(self.curr_state.clone());
                }
            }
            // if here, then full evaluation without changing state
            // program is finished
            self.finished = true;
            None
        }
    }
}

impl<T: FractranNat> Evaluator<T> {
    /// Constructs an Evaluator from a Program and a starting state. Panics if
    /// the given program is empty.
    pub fn new(program: Vec<Fraction<T>>, input: T) -> impl Iterator<Item = T> {
        if program.is_empty() {
            panic!("Cannot run empty program!");
        }
        Evaluator {
            program,
            curr_state: input,
            finished: false,
        }
    }
}

impl<T: FractranNat> Program<T> {
    /// Returns an iterator that lazily executes the program using a single
    /// input, stopping if the program halts.
    pub fn lazy_exec(self, input: T) -> impl Iterator<Item = T> {
        Evaluator::new(self.fracs, input)
    }

    /// Returns the final output of the program: this will obviously never
    /// terminate if the program itself doesn't.
    pub fn exec_to_completion(self, input: T) -> T {
        self.lazy_exec(input)
            .inspect(|step| {
                dbg!(step);
            })
            .last()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primebasis::PrimeBasis;

    /// Given two vectors of numerators and denominators initializes the program.
    fn make_program(nums: Vec<u64>, denoms: Vec<u64>) -> Program<PrimeBasis> {
        let mut prog: Vec<Fraction<PrimeBasis>> = vec![];
        for (num, denom) in nums.into_iter().zip(denoms) {
            prog.push(Fraction::new(
                PrimeBasis::try_new(num).unwrap(),
                PrimeBasis::try_new(denom).unwrap(),
            ));
        }
        Program { fracs: prog }
    }

    #[test]
    fn test_basic_program() {
        let div_then_stop = Program {
            fracs: vec![Fraction::new(1_u64, 2_u64)],
        };
        let mut iter = div_then_stop.clone().lazy_exec(4_u64);
        assert_eq!(iter.next(), Some(2_u64));
        assert_eq!(iter.next(), Some(1_u64));
        assert_eq!(iter.next(), None);
        assert_eq!(div_then_stop.exec_to_completion(4_u64), 1_u64);
    }

    #[test]
    fn test_multiply() {
        let mult_pb = make_program(vec![455, 11, 1, 3, 11, 1], vec![33, 13, 11, 7, 2, 3]);

        // here, we input 2^3 * 3^2, so we should get 5^6
        assert_eq!(
            mult_pb
                .exec_to_completion(PrimeBasis::try_new(72).unwrap())
                .value(),
            5_u64.pow(6)
        );
    }
    #[test]
    fn test_readme_primes() {
        let nums: Vec<u64> = vec![17, 78, 19, 23, 29,
                                  77, 95, 77, 1, 11,
                                  13, 15, 15, 55];
        let denoms: Vec<u64> = vec![91, 85, 51, 38, 33,
                                    29, 23, 19, 17, 13,
                                    11, 14, 2, 1];
        let fracs: Vec<Fraction<u64>> = nums.into_iter()
                                            .zip(denoms)
                                            .map(|(num, denom)| Fraction::new(num, denom))
                                            .collect();

        let prog = Program::new(fracs);
        let mut primes = vec![];
        for out in prog.lazy_exec(2).take(2000) {
            if out.is_power_of_two() {
                primes.push(out.trailing_zeros());
            }
        }
        assert_eq!(primes, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_readme_primes_2e() {
        let nums: Vec<u64> = vec![17, 78, 19, 23, 29,
                                  77, 95, 77, 1, 11,
                                  13, 15, 15, 55];
        let denoms: Vec<u64> = vec![91, 85, 51, 38, 33,
                                    29, 23, 19, 17, 13,
                                    11, 14, 2, 1];
        let fracs: Vec<Fraction<PrimeBasis>> = nums.into_iter()
                                                   .zip(denoms)
                                                   .map(|(num, denom)| Fraction::new(
                                                       PrimeBasis::try_new(num).unwrap(),
                                                       PrimeBasis::try_new(denom).unwrap(),
                                                   ))
                                                   .collect();

        let prog = Program::new(fracs);
        let mut primes = vec![];
        for out_pb in prog.lazy_exec(PrimeBasis::try_new(2).unwrap())
                          .take(100_000) {
            if out_pb.exps[1..].iter().all(|&exp| exp == 0) {
                primes.push(out_pb.exps[0]);
            }
        }
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]);
    }
}
