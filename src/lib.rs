pub mod problems;

pub mod divisors;
pub mod maximum_path_sum;
pub mod primes;
pub mod read_from_file;
pub mod symbolic_math;
pub mod utility;

pub fn run_problems(problem_numbers: &[u16]) {
    let prime_vec = primes::get_primes();
    for &num in problem_numbers {
        println!("{}: {}", num, run_problem(num, &prime_vec));
    }
}

fn run_problem(problem_number: u16, prime_vec: &Vec<u32>) -> i64 {
    match problem_number {
        1 => problems::p01() as i64,
        2 => problems::p02() as i64,
        3 => problems::p03(prime_vec) as i64,
        4 => problems::p04() as i64,
        5 => problems::p05(prime_vec) as i64,
        6 => problems::p06() as i64,
        7 => problems::p07(prime_vec) as i64,
        8 => problems::p08() as i64,
        9 => problems::p09() as i64,
        10 => problems::p10(prime_vec) as i64,
        11 => problems::p11() as i64,
        12 => problems::p12(prime_vec) as i64,
        13 => problems::p13() as i64,
        14 => problems::p14() as i64,
        15 => problems::p15() as i64,
        16 => problems::p16() as i64,
        17 => problems::p17() as i64,
        18 => problems::p18() as i64,
        19 => problems::p19() as i64,
        20 => problems::p20() as i64,
        21 => problems::p21(prime_vec) as i64,
        22 => problems::p22() as i64,
        23 => problems::p23(prime_vec) as i64,
        24 => problems::p24() as i64,
        25 => problems::p25() as i64,
        26 => problems::p26() as i64,
        27 => problems::p27(prime_vec),
        28 => problems::p28() as i64,
        29 => problems::p29(prime_vec) as i64,
        30 => problems::p30() as i64,
        31 => problems::p31() as i64,
        _ => 42,
    }
}
