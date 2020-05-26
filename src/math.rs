//! A module providing mathematical utility functions required for operating
//! FRACTRAN code.

/// Computes the first n primes as a `Vec` using the Sieve of Eratosthenes.
pub fn first_n_primes(n: u16) -> Vec<u64> {
    // for all n >= 6, p_n < n(log n + log log n)
    // otherwise, because p_5 = 11, p_n < 11
    let max_prime: usize = if n < 6 {
        11
    } else {
        (n as f64 * ((n as f64).ln() + (n as f64).ln().ln())).floor() as usize
    };

    let mut primes: Vec<bool> = vec![true; max_prime as usize + 1];
    primes[0] = false;
    primes[1] = false;
    for i in 2..=max_prime {
        if primes[i] {
            let mut curr_mult = i * 2;
            while curr_mult <= max_prime {
                primes[curr_mult] = false;
                curr_mult += i;
            }
        }
    }

    return primes
        .into_iter()
        .enumerate()
        .filter(|(_, is_p)| *is_p)
        .map(|(i, _)| i as u64)
        .take(n as usize)
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_n_primes_above_6() {
        assert_eq!(
            first_n_primes(12),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
        );
    }

    #[test]
    fn first_n_primes_below_6() {
        assert_eq!(first_n_primes(5), vec![2, 3, 5, 7, 11]);

        assert_eq!(first_n_primes(1), vec![2]);

        assert_eq!(first_n_primes(0), vec![]);
    }
}
