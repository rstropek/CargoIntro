unsafe extern "C" { fn calculateNumberOfSolutions(boardSize: i32) -> i32; }

fn main() {
    let solutions: i32;
    unsafe { solutions = calculateNumberOfSolutions(8); };
    println!("{:?}", solutions);
}