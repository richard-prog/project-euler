pub mod problems;

pub mod divisors;
pub mod maximum_path_sum;
pub mod primes;
pub mod read_from_file;
pub mod symbolic_math;

pub fn run_problems(problem_numbers: &[u8]) {
    // let problems = Vec::new();
    // problems.push(problems::p01 as fn() -> u64);
    // problems.push(problems::p02 as fn() -> u64);
    let problems = vec![
    	problems::p01 as fn() -> u64,
    	problems::p02 as fn() -> u64,
    	problems::p03 as fn() -> u64,
    	problems::p04 as fn() -> u64,
    	problems::p05 as fn() -> u64,
    	problems::p06 as fn() -> u64,
    	problems::p07 as fn() -> u64,
    	problems::p08 as fn() -> u64,
    	problems::p09 as fn() -> u64,
    	problems::p10 as fn() -> u64,
    	problems::p11 as fn() -> u64,
    	problems::p12 as fn() -> u64,
    	problems::p13 as fn() -> u64,
    	problems::p14 as fn() -> u64,
    	problems::p15 as fn() -> u64,
    	problems::p16 as fn() -> u64,
    	problems::p17 as fn() -> u64,
    	problems::p18 as fn() -> u64,
	problems::p19 as fn() -> u64,
	problems::p20 as fn() -> u64,
	problems::p25 as fn() -> u64,
    ];
    if problem_numbers.len() == 0 {
	for (num, problem) in problems.iter().enumerate() {
	    println!("{}: {}", num, problem());
	}
    } else {
	for num in problem_numbers {
	    println!("{}: {}", num, problems[*num as usize - 1]());
	}
    }
}
