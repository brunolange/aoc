use itertools::iterate;

const INITIAL_VALUE: usize = 20_151_125;
const MULTIPLIER: usize = 252_533;
const MODULUS: usize = 33_554_393;

fn code(row: usize, column: usize) -> usize {
    let max_row = row + column - 1;
    let steps_to_max_row = max_row * (max_row - 1) / 2; // max_row choose 2
    let steps = steps_to_max_row + column;

    iterate(INITIAL_VALUE, |curr| (curr * MULTIPLIER) % MODULUS)
        .take(steps)
        .last()
        .unwrap()
}

fn main() {
    println!("{}", code(3_010, 3_019));
}
