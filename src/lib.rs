pub mod problems;

pub mod divisors;
pub mod maximum_path_sum;
pub mod primes;
pub mod read_from_file;
pub mod symbolic_math;

pub fn run_problems(problem_numbers: &[u16]) {
    let primes = primes::get_primes();
    for &num in problem_numbers {
	println!("{}: {}", num, run_problem(num, &primes));
    }
}

fn run_problem(problem_number: u16, primes: &Vec<u32>) -> u64 {
    match problem_number {
	1 => problems::p01(),
	2 => problems::p02(),
	3 => problems::p03(),
	4 => problems::p04(),
	5 => problems::p05(&primes),
	6 => problems::p06(),
	7 => problems::p07(&primes),
	8 => problems::p08(),
	9 => problems::p09(),
	10 => problems::p10(&primes),
	11 => problems::p11(),
	12 => problems::p12(&primes),
	13 => problems::p13(),
	14 => problems::p14(),
	15 => problems::p15(),
	16 => problems::p16(),
	17 => problems::p17(),
	18 => problems::p18(),
	19 => problems::p19(),
	20 => problems::p20(),
	21 => problems::p21(&primes),
	22 => problems::p22(),
	23 => problems::p23(&primes),
	24 => problems::p24(),
	25 => problems::p25(),
	_ => 42
    }
}
