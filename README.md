## Investigating Prime Sequences 

The code in this repository implements a series of primality tests in Rust and powers a command-line utility to compute prime sequences and display them on graphs.

The distribution of primes and related numbers is not random, certain properties of the corresponding sequences can be visualized by drawing graphs such as the [Ulam Spiral](https://en.wikipedia.org/wiki/Ulam_spiral), such as this image marking the prime numbers from 1 to 1000000 in a spiral arrangement on a square lattice:

![Ulam Spiral](Ulam_Spiral.png)

Recurring elements such as lines are visible. This software package allows for performing experimental mathematics on this topic, i.e. studying arbitrary sequences to ideentify properties and patterns.


## Setup

Install Rust:
- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- or update it with `rustup update`

Build the software package:
- `cargo build`
- `cargo test`
- `cargo run [arg1] [arg2]`

Clean the directory:
- `cargo clean`


## Primality tests

The following types are available

- Primes: `primes`
- Pythagorean primes: `pythagorean`
- Semiprimes: `semiprimes`
- Squarefree semiprimes: `squarefree-semiprimes`
- Pernicious numbers: `pernicious`
- Prime powers: `prime-powers`
- Fermi-Dirac primes: `fermi-dirac`


## Usage

On the command-line, unless the default is used all arguments have to be defined, argument order matters:

1. Number type (primes), see alternatives above
2. Start (1)
3. End (1000000)
4. Output type (b-file), alternative: graph

Examples:

- `cargo run semiprimes 1 10` (outputs the first 10 semiprimes in the terminal)
- `cargo run primes 1 100000 graph` (generates two graphs, a sequential one and an Ulam spiral)
