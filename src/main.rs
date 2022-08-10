use std::env;

fn main() {
    let problems = env::args()
	.skip(1)
	.map(|x| x.parse::<u16>().unwrap())
	.collect::<Vec<u16>>();
    project_euler::run_problems(&problems);
}
