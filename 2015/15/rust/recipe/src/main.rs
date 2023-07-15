use recipe::maximize_score;
use recipe::Ingredient;

mod io;

fn main() {
    env_logger::init();

    let ingredients: Vec<Ingredient> = io::lines().map(io::parse_line).collect();
    let (max_score, amounts) = maximize_score(&ingredients);

    for amount in amounts {
        println!("{}: {}", amount.ingredient.name, amount.quantity);
    }

    println!("Score = {:?}", max_score);
}
