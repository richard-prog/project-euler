use std::error::Error;
use std::{env, fmt, fs};

pub mod problems;

pub mod divisors;
pub mod maximum_path_sum;
pub mod primes;
pub mod read_from_file;
pub mod symbolic_math;
pub mod utility;

pub fn run_problems(problem_numbers: &[u16]) {
    let (prime_sieve, prime_vec) = primes::generate_primes();
    println!("{:?}", prime_vec);
    for &num in problem_numbers {
        println!("{}: {}", num, run_problem(num, &prime_sieve, &prime_vec));
    }
}

fn run_problem(problem_number: u16, prime_sieve: &[bool], prime_vec: &Vec<u32>) -> Solution {
    match problem_number {
        1 => Solution::Unsigned(problems::p01()),
        2 => Solution::Unsigned(problems::p02()),
        3 => Solution::Unsigned(problems::p03(prime_vec)),
        4 => Solution::Unsigned(problems::p04()),
        5 => Solution::Unsigned(problems::p05(prime_vec)),
        6 => Solution::Unsigned(problems::p06()),
        7 => Solution::Unsigned(problems::p07(prime_vec)),
        8 => Solution::Unsigned(problems::p08()),
        9 => Solution::UnsignedResult(problems::p09()),
        10 => Solution::Unsigned(problems::p10(prime_vec)),
        11 => Solution::UnsignedResult(problems::p11()),
        12 => Solution::Unsigned(problems::p12(prime_vec)),
        13 => Solution::UnsignedResult(problems::p13()),
        14 => Solution::Unsigned(problems::p14()),
        15 => Solution::Unsigned(problems::p15()),
        16 => Solution::Unsigned(problems::p16()),
        17 => Solution::Unsigned(problems::p17()),
        18 => Solution::UnsignedResult(problems::p18()),
        19 => Solution::Unsigned(problems::p19()),
        20 => Solution::Unsigned(problems::p20()),
        21 => Solution::Unsigned(problems::p21(prime_vec)),
        22 => Solution::UnsignedResult(problems::p22()),
        23 => Solution::Unsigned(problems::p23(prime_vec)),
        24 => Solution::UnsignedResult(problems::p24()),
        25 => Solution::Unsigned(problems::p25()),
        26 => Solution::Unsigned(problems::p26()),
        27 => Solution::Signed(problems::p27(prime_vec)),
        28 => Solution::Unsigned(problems::p28()),
        29 => Solution::Unsigned(problems::p29(prime_vec)),
        30 => Solution::Unsigned(problems::p30()),
        31 => Solution::UnsignedResult(problems::p31()),
        32 => Solution::Unsigned(problems::p32()),
        33 => Solution::Unsigned(problems::p33()),
        34 => Solution::Unsigned(problems::p34()),
        35 => Solution::Unsigned(problems::p35(prime_sieve, prime_vec)),
        problem_number => Solution::None { problem_number },
    }
}

enum Solution {
    Unsigned(u64),
    Signed(i64),
    UnsignedResult(Result<u64, Box<dyn Error>>),
    None { problem_number: u16 },
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Solution::Unsigned(x) => write!(f, "{}", x),
            Solution::Signed(x) => write!(f, "{}", x),
            Solution::UnsignedResult(x) => match x {
                Ok(x) => write!(f, "{}", x),
                Err(e) => write!(f, "Error: {}", e),
            },
            Solution::None { problem_number } => {
                write!(f, "Haven't solved problems number {}", problem_number)
            }
        }
    }
}

pub fn get_problems(mut args: env::Args) -> Result<Vec<u16>, Box<dyn Error>> {
    let mut problems = Vec::new();
    args.next();
    let first_arg = args
        .next()
        .ok_or_else(|| "no problems to run".to_string())?;
    if first_arg == "all" {
        let paths = fs::read_dir("src/problems/")?;
        for path in paths {
            let filename = &path?
                .path()
                .to_str()
                .ok_or_else(|| String::from("Filename with invalid unicode"))?
                .to_string();
            if filename.as_bytes()[filename.len() - 1] == b'~' {
                continue;
            }
            let num = filename[14..16].parse::<u16>()?;
            problems.push(num);
        }
    } else {
        problems.push(first_arg.parse::<u16>()?);
        for arg in args {
            problems.push(arg.parse::<u16>()?);
        }
    }
    problems.sort_unstable();
    Ok(problems)
}
