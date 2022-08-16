use std::env;
use std::process;

use project_euler::{get_problems, run_problems};

fn main() {
    let problems = get_problems(env::args()).unwrap_or_else(|err| {
	eprintln!("Error: {}", err);
	process::exit(1);
    });
    run_problems(&problems);
}
