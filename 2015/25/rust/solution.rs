fn code(row: usize, column: usize) -> usize {
    let max_row = row + column - 1;
    let steps_to_max_row = max_row * (max_row-1) / 2;  // max_row choose 2
    let steps = steps_to_max_row + column - 1;

    let mut curr = 20_151_125;

    for _ in 0..steps {
        curr *= 252_533;
        curr %= 33_554_393;
    }

    curr
}

fn main() {
    println!("{}", code(3_010, 3_019));
}

