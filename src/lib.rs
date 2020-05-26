//! A Fractran interpreter written in Rust.

#[macro_use]
extern crate lazy_static;

mod math;

/// N, where the nth prime is the largest one allowed as a factor of an input:
/// intuitively, the number of registers the program can read and write to. For
/// a value of `1000`, this means that the first number that cannot be expressed
/// is `7927`.
pub const MAX_REGS: u16 = 1000;

lazy_static! {
    /// The list of the first `MAX_REGS` primes, generated at run time from
    /// `MAX_REGS`.
    pub static ref PRIMES: Vec<u64> = math::first_n_primes(MAX_REGS);
}

mod frac;
mod primebasis;
mod program;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
