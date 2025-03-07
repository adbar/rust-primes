fn sqrt_ceil(n: usize) -> usize {
    (n as f64).sqrt() as usize + 1
}


// see also https://en.wikipedia.org/wiki/Wheel_factorization
pub fn is_prime(n: usize) -> bool { // https://oeis.org/A000040
    if n <= 3 {
        if n <= 1 {
            return false; // 0 and 1 are not prime
        }
        return true; // 2 and 3 are prime
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false; // eliminate multiples of 2 and 3
    }

    // check for factors of the form 6k ± 1
    for i in (5..sqrt_ceil(n)).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }

    true
}


pub fn is_pythagorean_prime(n: usize) -> bool { // https://oeis.org/A002144
    is_prime(n) && n % 4 == 1 // 4n + 1
}


fn _is_semiprime(n: usize, square_free: bool) -> bool {
    if n < 4 {
        return false;
    }

    for i in 2..sqrt_ceil(n) {
        if n % i == 0 {
            let factor = n / i;
            if is_prime(i) && is_prime(factor) {
                return !square_free || factor != i;
            }
            break;
        }
    }

    false
}


pub fn is_semiprime(n: usize) -> bool { // https://oeis.org/A001358
    _is_semiprime(n, false)
}


pub fn is_squarefree_semiprime(n: usize) -> bool { // https://oeis.org/A006881
    _is_semiprime(n, true)
}


pub fn is_pernicious(n: usize) -> bool { // https://oeis.org/A052294
    is_prime(n.count_ones() as usize)
}


pub fn is_prime_power(n: usize) -> bool { // https://oeis.org/A000961
    if n == 1 {
        return true;
    }

    for i in 2..sqrt_ceil(n) {
        let mut power = 1;
        while i.pow(power) <= n {
            if i.pow(power) == n && is_prime(i) {
                return true;
            }
            power += 1;
        }
    }

    is_prime(n)
}


pub fn is_fermi_dirac(n: usize) -> bool { // https://oeis.org/A050376

    for b in 2..sqrt_ceil(n) {
        if is_prime(b) {
            let mut e = 2;
            while b.pow(e) <= n {
                if b.pow(e) == n && (e & (e - 1)) == 0 {
                    return true;
                }
                e += 1;
            }
        }
    }

    is_prime(n)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_test() {
        let test_cases = vec![
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (71, true),
            (73, true),
            (75, false),
            (101, true),
            (102, false),
            (41041, false),
            (43201, true),
        ];
        for (input, expected) in test_cases {
            assert_eq!(is_prime(input), expected);
        }
    }

    #[test]
    fn pythagorean_primes_test() {
        let test_cases = vec![
            (0, false),
            (1, false),
            (2, false),
            (3, false),
            (4, false),
            (5, true),
            (6, false),
            (7, false),
            (11, false),
            (13, true),
            (107, false),
            (113, true),
            (593, true),
            (601, true),
        ];
        for (input, expected) in test_cases {
            assert_eq!(is_pythagorean_prime(input), expected);
        }
    }

    #[test]
    fn sqrt_test() {
        assert_eq!(sqrt_ceil(9), 4);
        assert_eq!(sqrt_ceil(10), 4);
    }

    #[test]
    fn semiprimes_test() {
        let test_cases = vec![
            (0, false),
            (1, false),
            (2, false),
            (3, false),
            (4, true),
            (5, false),
            (6, true),
            (7, false),
            (8, false),
            (9, true),
            (10, true),
            (14, true),
            (51, true),
            (166, true),
            (167, false),
            (168, false),
            (169, true),
        ];
        for (input, expected) in test_cases {
            assert_eq!(is_semiprime(input), expected);
        }
    }

    #[test]
    fn squarefree_semiprimes_test() {
        let test_cases = vec![
            (0, false),
            (1, false),
            (2, false),
            (3, false),
            (4, false),
            (5, false),
            (6, true),
            (7, false),
            (10, true),
            (51, true),
            (198, false),
            (199, false),
            (200, false),
            (201, true),
            (202, true),
            (203, true),
        ];
        for (input, expected) in test_cases {
            assert_eq!(is_squarefree_semiprime(input), expected);
        }
    }

    #[test]
    fn pernicious_test() {
        let test_cases = vec![
            (0, false),
            (1, false),
            (2, false),
            (3, true),
            (4, false),
            (5, true),
            (6, true),
            (7, true),
            (8, false),
            (9, true),
            (10, true),
            (95, false),
            (96, true),
            (98, true),
            (99, false),
        ];

        for (input, expected) in test_cases {
            assert_eq!(is_pernicious(input), expected);
        }
    }

    #[test]
    fn power_test() {
        let test_cases = vec![
            (0, false),
            (1, true),
            (2, true),
            (3, true),
            (4, true),
            (5, true),
            (6, false),
            (7, true),
            (81, true),
            (82, false),
            (121, true),
            (125, true),
            (126, false),
            (127, true),
            (128, true),
            (129, false),
            (5329, true),
        ];

        for (input, expected) in test_cases {
            assert_eq!(is_prime_power(input), expected);
        }
    }

    #[test]
    fn fermi_dirac_test() {
        let test_cases = vec![
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, true),
            (5, true),
            (6, false),
            (7, true),
            (82, false),
            (83, true),
            (121, true),
            (127, true),
            (128, false),
        ];

        for (input, expected) in test_cases {
            assert_eq!(is_fermi_dirac(input), expected);
        }
    }
}
