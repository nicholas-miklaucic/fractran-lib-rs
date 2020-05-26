# Fractran

This is a library that implements the [FRACTRAN](https://en.wikipedia.org/wiki/FRACTRAN) esoteric programming language,
invented by John Conway, in Rust. It supplies a basic way of running programs in
FRACTRAN and additionally implements a custom numeric type that is better suited
for the language than traditional types like `u64`.

This is a low-level crate: as you&rsquo;ll see below, there&rsquo;s a lot of boilerplate
involved in actually running a program. Stay tuned for a crate that wraps this
and provides a simple CLI for running FRACTRAN programs.


<a id="org87ce20c"></a>

# Quickstart


<a id="orgdbea104"></a>

## Sample Program

This is one of Conway&rsquo;s original programs, used to generate the prime numbers.
They appear as the exponents of the powers of 2 that the program produces.

```rust
// make the list of fractions
let nums: Vec<u64> = vec![17, 78, 19, 23, 29,
                          77, 95, 77, 1, 11,
                          13, 15, 15, 55];
let denoms: Vec<u64> = vec![91, 85, 51, 38, 33,
                            29, 23, 19, 17, 13,
                            11, 14, 2, 1];
                            
// Fraction is generic because it can use types other than u64, as we'll see shortly
    
// this is probably the easiest way to make a big list of fractio
let fracs: Vec<Fraction<u64>> = nums.into_iter()
                                    .zip(denoms)
                                    .map(|(num, denom)| Fraction::new(num, denom))
                                    .collect();
    
// create the program from that list
let prog = Program::new(fracs);
let mut primes = vec![];
    
// lazy_exec() returns an iterator that runs as long as the program does, which in this case is forever
// we fix that by stopping early using take()
// note also that we feed in 2
for out in prog.lazy_exec(2).take(2000) {
    // Rust's integer methods are pretty neat!
    if out.is_power_of_two() {
        primes.push(out.trailing_zeros());
    }
}
assert_eq!(primes, vec![2, 3, 5, 7]);
```

<a id="org0256a15"></a>

## Dealing with Overflow

You might notice that this only prints out 4 primes in 2000 executions. As it
turns out, when you can only use a single instruction, programs run long!
Additionally, the numbers can get big really quickly. `u64` will overflow before
you can output `11`: try replacing `.take(2000)` with a larger number to see what I
mean.

To fix the overflow issue, this crate has a `PrimeBasis` type that behaves like
`u64` in the ways that FRACTRAN needs, but stores the prime factorization of each
number instead of the actual value. Because of the way FRACTRAN works, it&rsquo;s very
rare to use large primes, and so in practice this is a huge space improvement.

We can modify the above program pretty easily to use `PrimeBasis`, which lets us
compute as many primes as we like.

```rust
let nums: Vec<u64> = vec![17, 78, 19, 23, 29,
                          77, 95, 77, 1, 11,
                          13, 15, 15, 55];
let denoms: Vec<u64> = vec![91, 85, 51, 38, 33,
                            29, 23, 19, 17, 13,
                            11, 14, 2, 1];
    
// we have to now create a PrimeBasis, which adds a little more boilerplate here
// try_new() returns a Result because there's a fixed number of primes that the program uses
// any number that requires a factor not in our list of primes can't be represented
// this is almost never an issue because it's very rare you want to use very large prime factors
let fracs: Vec<Fraction<PrimeBasis>> = nums.into_iter()
                                           .zip(denoms)
                                           .map(|(num, denom)| Fraction::new(
                                               PrimeBasis::try_new(num).unwrap(),
                                               PrimeBasis::try_new(denom).unwrap(),
                                           ))
                                           .collect();
    
let prog = Program::new(fracs);
let mut primes = vec![];
    
// now can run the program for a lot longer: 100,000 iterations is pretty fast on my machine still
// note that we need to feed in a PrimeBasis because that's the type our program is in
for out_pb in prog.lazy_exec(PrimeBasis::try_new(2).unwrap())
                  .take(100_000) {
    // we no longer have the integer methods, so we directly work with the list of exponents
    if out_pb.exps[1..].iter().all(|&exp| exp == 0) {
        primes.push(out_pb.exps[0]);
    }
}
// we have a lot more primes now!
assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]);
```

<a id="org45a70c5"></a>

# Notes

-   This is hardly optimized, but it isn&rsquo;t slow either, especially when using
    `PrimeBasis`. Each operation is essentially doing some very simple iteration
    over the number of prime factors being used. For programs where the largest
    prime factor is small, which is most programs, a single operation is going to
    be on the order of 10-20 basic operations.
-   The number of primes allowed in a `PrimeBasis` is exported as `MAX_REGS`. It&rsquo;s
    currently `1000`, which is a lot. (The name is because it&rsquo;s easiest to think of
    each prime exponent as a register in a more traditional program: this is the
    number of registers.)
-   The list of primes used by `PrimeBasis`, in order, is exported as `PRIMES`: it&rsquo;s
    generated at compile time using `MAX_REGS` and a basic sieve. Use this and
    `MAX_REGS` instead of hardcoding if it&rsquo;s necessary.
-   Note that `PrimeBasis` doesn&rsquo;t store extra trailing zeroes in its list of
    exponents, so there&rsquo;s no space cost if you aren&rsquo;t using the extra register
    space. The reason I don&rsquo;t let you use a million registers is because that
    would adversely affect compilation time.


<a id="orgf06d1e1"></a>

# Contributing/Issues

-   If you have an issue, feature request, or anything like that, feel free to
    file one.
-   I would elaborate, but I honestly can&rsquo;t imagine what anyone&rsquo;s use case for
    this would be and therefore can&rsquo;t really imagine anyone having issues or
    feature requests.

