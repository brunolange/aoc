use recipe::maximum_score;
use recipe::Ingredient;

mod io;

fn main() {
    env_logger::init();

    let ingredients: Vec<Ingredient> = io::lines().map(io::parse_line).collect();
    let (max_score, amounts) = maximum_score(&ingredients);

    amounts.iter().for_each(|amount| {
        println!("{:.<20}: {}", amount.ingredient.name, amount.quantity);
    });

    println!("{:?}", max_score);
}
