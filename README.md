# project-euler

My solutions to the first several Project Euler problems in Rust.
The ultimate goal is to solve the first 100, as Project Euler grants for "for those problems and their solutions to be discussed elsewhere."

To run a problem (e.g., 25) with Cargo, run `cargo run --release 25`.

To run all problems with Cargo, run `cargo run all`.

The following principles inform my solutions:
1. **Solve the most general possible problem possible**. For instance, problem 2 asks you to sum even Fibonacci numbers at most 4,000,000.
   A more general solution will sum Fibonacci numbers below an arbitrary 64-bit unsigned integer.
2. **Maximize test coverage**. Unless I missed something, I have all problems and libraries used therein tested.
   Solving the problem in maximum generality creates excellent opportunities for testing; I typically get correct solution on first submission.
3. **Eschew `unsafe`**. There is a time and place for unsafe Rust and it is not here.
4. **Performance**. Rust goes *foom*. Project Euler affords many opportunities to go *foom*.
   It has not escaped my attention that, in a production environment, taking a function from 500 nanoseconds to 120 nanoseconds usually not an effective use of programmer time.
   That said, it's *fun*.
