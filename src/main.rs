fn get_primes() -> Vec<u128> {
    let mut res = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
    ];
    // res.reverse();
    return res;
}

fn find_q_numbers(p: u128) -> Vec<u128> {
    let primes = get_primes();
    let q_numbers: Vec<u128> = (2..=p).into_iter().filter(|q| (p - 1) % q == 0).collect();
    return q_numbers;

    return vec![];
}

fn main() {
    let p = 97;

    let q_numbers = find_q_numbers(p);
    println!("q_numbers = {:?}", q_numbers);
    // let found_alfa: Vec<Option<u128>> = (2..p)
    // let found_alfa: Vec<(u128, bool)> = (2..p)
    let found_alfas: Vec<u128> = (2..p)
        .into_iter()
        .map(|alfa| {
            (
                alfa,
                q_numbers
                    .clone()
                    .into_iter()
                    .all(|q| potegowanie(alfa, (p - 1) / q, p) != 1),
            )
        })
        .filter(|(_alfa, fits_predicate)| *fits_predicate)
        .map(|(alfa, _fits_predicate)| alfa)
        // .flatten()
        .collect();
    // .collect();
    println!("found alfas: {:?}", found_alfas);
    let first_alfa = found_alfas.get(0);
    println!("first alfa = {:?}", first_alfa);
    // .collect();

    // println!("Hello, world!");
}

fn potegowanie(a: u128, e: u128, n: u128) -> u128 {
    let e_binary = reverse(create_binary(e));
    let mut d = 1;
    let mut i: i128 = e_binary.len() as i128 - 1;
    while (i >= 0) {
        d = modulo_euclid(d * d, n as i128);
        if e_binary[i as usize] == 1 {
            d = modulo_euclid(d * a as i128, n as i128)
        }

        i -= 1;
    }
    return d as u128;
}

fn create_binary(value: u128) -> Vec<u128> {
    let binary_string = format!("{:b}", value);
    let res = binary_string
        .chars()
        .into_iter()
        .map(|c| if c == '0' { 0 } else { 1 })
        .collect();
    return res;
}

fn reverse(vec: Vec<u128>) -> Vec<u128> {
    let mut vec_clone = vec.clone();
    vec_clone.reverse();
    // vec_clone.sort_by_key(|&num| (false , Reverse(num)));
    return vec_clone;
}

fn modulo_euclid(j: i128, k: i128) -> i128 {
    let res = j % k;
    if res < 0 {
        return res + k;
    } else {
        return res;
    }
}
