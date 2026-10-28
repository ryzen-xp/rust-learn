// Topic: Tic-tac-toe-checker
// Run with:  cargo run --bin Tic-tac-toe-checker

fn main() {
    println!("Hello World : cargo run --bin Tic-tac-toe-checker");
}

/*

[[0, 0, 1],               where, 0 => Empty , 1=> X , 2=> O
 [0, 1, 2],              return   if Not won any one = -1 , X won => 1  , O won => 2 , draw => 0
 [2, 1, 0]]

*/

fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    todo!()
}
