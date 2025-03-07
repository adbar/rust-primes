use rayon::prelude::*;

mod primality;



pub fn get_function(ntype: &str) -> Option<fn(usize) -> bool> {
    // function_a+function_b ?
    match ntype {
        "primes" => Some(primality::is_prime),
        "pythagoreans" => Some(primality::is_pythagorean_prime),
        "semiprimes" => Some(primality::is_semiprime),
        "squarefree-semiprimes" => Some(primality::is_squarefree_semiprime),
        "pernicious" => Some(primality::is_pernicious), 
        "prime-powers" => Some(primality::is_prime_power),
        "fermi-dirac" => Some(primality::is_fermi_dirac),
        _ => None,
    }
}


fn define_functions(input: &str) -> (Vec<fn(usize) -> bool>, bool) {
    let splitchar = if input.contains("X") { "X" } else { "+" };
    let functions = input.split(splitchar)
         .filter_map(get_function)
         .collect();
    (functions, splitchar == "X")
}


fn is_valid_number(n: usize, functions: &[fn(usize) -> bool], switch: bool) -> bool {
    if !switch {
        functions.iter().any(|func| func(n))
    } else {
        functions.iter().all(|func| func(n))
    }
}


pub fn move_around(grid_size: isize, n: usize) -> Vec<(isize, isize)> {
    generate_spiral_tuples(grid_size, n).par_iter()
        .filter_map(|(i, x, y)| primality::is_prime(*i).then_some((*x, *y)) )
        .collect()
}


pub fn move_in_line(grid_size: isize, n: usize) -> Vec<(isize, isize)> {
    generate_line_tuples(grid_size, n).par_iter()
        .filter_map(|(i, x, y)| primality::is_prime(*i).then_some((*x, *y)) )
        .collect()
}


pub fn compute_numbers_limit(start: usize, ntype: &str, limit: usize) -> Vec<usize> {
    let (functions, switch) = define_functions(ntype);

    (start..)
        .filter(|&num| is_valid_number(num, &functions, switch))
        .take(limit)
        .collect()
}


pub fn compute_numbers_end(start: usize, ntype: &str, end: usize) -> Vec<usize> {
    let (functions, switch) = define_functions(ntype);

    let numbers: Vec<_> = (start..end).collect();
    numbers
        .par_iter()
        .filter(|&num| is_valid_number(*num, &functions, switch))
        .cloned()
        .collect()
}


pub fn print_first_ones(start: usize, ntype: &str) {
    let numbers = compute_numbers_limit(start, ntype, 10);
    println!("First {}: {:?}", ntype, numbers);
}


fn generate_line_tuples(grid_size: isize, mut n: usize) -> Vec<(usize, isize, isize)> {
    let mut tuples = Vec::with_capacity((grid_size as usize).pow(2));

    for y in 0..=grid_size {
        for x in 0..=grid_size {
            tuples.push((n, x, y));
            n += 1;
        }
    }

    tuples
}


fn generate_spiral_tuples(grid_size: isize, mut n: usize) -> Vec<(usize, isize, isize)> {
    let mut tuples = Vec::with_capacity((grid_size as usize).pow(2));

    let mut x: isize = 0;
    let mut y: isize = 0;
    let directions: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)]; // right, up, left, down
    let mut direction = 0;
    let mut step = 1;
    let mut m = 1;

    while x <= grid_size && y <= grid_size {
        //println!("{:?}", (x, y, direction, step));
        tuples.push((n, x, y));

        x += directions[direction].0;
        y += directions[direction].1;

        if m % step == 0 { // cycle through directions
            direction = (direction + 1) % 4;
            if direction % 2 == 0 { // increase step size after two turns
                step += 1;
            }
        }
        n += 1;
        m += 1;
    }
    tuples
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_function() {
        assert_eq!(get_function("unknown"), None);
    }

    #[test]
    fn test_define_functions() {
        assert_eq!(define_functions("primes").0.len(), 1);
        assert_eq!(define_functions("unknown").0.len(), 0);
        assert_eq!(define_functions("primes+semiprimes").0.len(), 2);
        assert_eq!(define_functions("primes+unknown").0.len(), 1);
        let (functions, contains_x) = define_functions("primesXpernicious");
        assert_eq!(functions.len(), 2);
        assert!(contains_x);
    }

    #[test]
    fn test_valid_number() {
        assert!(define_functions("").0.is_empty());

        let (functions, switch) = define_functions("primes");
        assert_eq!(is_valid_number(2, &functions, switch), true);
        assert_eq!(is_valid_number(4, &functions, switch), false);

        let (functions, switch) = define_functions("semiprimes");
        assert_eq!(is_valid_number(4, &functions, switch), true);

        let (functions, switch) = define_functions("primes+semiprimes");
        assert_eq!(is_valid_number(4, &functions, switch), true);
    }

    #[test]
    fn test_compute_numbers() {
        let result = compute_numbers_limit(1, "primes", 5);
        assert_eq!(result, vec![2, 3, 5, 7, 11]);
        let result = compute_numbers_limit(1, "primes+squarefree-semiprimes", 5);
        assert_eq!(result, vec![2, 3, 5, 6, 7]);
        let result = compute_numbers_end(10, "primes", 20);
        assert_eq!(result, vec![11, 13, 17, 19]);
        let result = compute_numbers_end(10, "primes+squarefree-semiprimes", 12);
        assert_eq!(result, vec![10, 11]);
        let result = compute_numbers_end(1, "primes+pernicious", 10);
        assert_eq!(result, vec![2, 3, 5, 6, 7, 9]);
        let result = compute_numbers_end(1, "primesXpernicious", 10);
        assert_eq!(result, vec![3, 5, 7]);
    }

    #[test]
    fn coordinates_test() {
        let coordinates = move_around(2, 1);
        assert_eq!(coordinates[coordinates.len() -1], (0, -2));
        let coordinates = move_around(2, 1000);
        assert_eq!(coordinates[coordinates.len() -1], (-1, -2));
        let coordinates = move_in_line(2, 1);
        assert_eq!(coordinates[coordinates.len() -1], (0, 2));
    }
}
