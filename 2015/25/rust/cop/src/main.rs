use itertools::iterate;
use std::env::args;

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

fn code_2(row: usize, column: usize) -> usize {
    let max_row = row + column - 1;
    let steps_to_max_row = max_row * (max_row - 1) / 2; // max_row choose 2
    let steps = steps_to_max_row + column - 1;

    mod_ab_exp(INITIAL_VALUE, MULTIPLIER, steps, MODULUS)
}

fn mod_ab_exp(a: usize, b: usize, exp: usize, modulus: usize) -> usize {
    // uv mod m = (u mod m * v mod m) mod m
    let a_mod_m = a % modulus;
    let b_exp_mod_m = mod_exp(b, exp, modulus);
    (a_mod_m * b_exp_mod_m) % modulus
}

// Calculates b^e mod m
fn mod_exp(base: usize, exp: usize, modulus: usize) -> usize {
    if modulus == 1 {
        return 0;
    }
    let mut c = 1;
    let (mut b, mut e) = (base % modulus, exp);
    while e > 0 {
        if e % 2 == 1 {
            c = c * b % modulus
        }
        e >>= 1;
        b = b * b % modulus
    }
    c
}

fn main() {
    let solver = match args().nth(1).as_deref() {
        Some("2") => code_2,
        _ => code,
    };
    println!("{}", solver(3_010, 3_019));
}
