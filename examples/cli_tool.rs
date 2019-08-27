use sudoku::Board;

fn main() {
    let board = Board::from_text(
        &std::fs::read_to_string(&std::env::args().nth(1).expect("not enough args"))
            .expect("could not read file"),
    )
    .expect("couldnt parse");
    println!("board: \n{}", board);
    if let Some(soln) = board.solve(false).get(0) {
        println!("soln:\n{}", soln);
    } else {
        println!("no soln found");
    }
}
