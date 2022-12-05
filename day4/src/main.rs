use crate::puzzle_solver::PuzzleSolver;

mod puzzle_solver;

fn main() {
    let result = PuzzleSolver::solve();
    println!("{:?}", result);
}
