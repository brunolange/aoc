use recipe::{n_multichoose_k, score};
use recipe::{Amount, Ingredient};

mod io;

fn main() {
    let ingredients: Vec<Ingredient> = io::lines().map(io::parse_line).collect();
    let (max_score, amounts) = n_multichoose_k(100, ingredients.len())
        .iter()
        .map(|arrangement| {
            // println!("{:?}", arrangement);
            let amounts: Vec<_> = ingredients
                .iter()
                .zip(arrangement)
                .map(|(a, &b)| Amount {
                    quantity: b,
                    ingredient: a,
                })
                .collect();

            (score(&amounts), amounts)
        })
        .filter(|(score, _)| score.calories == 500)
        .max_by_key(|(score, _amounts)| score.value)
        .unwrap();

    for amount in amounts {
        println!("{}: {}", amount.ingredient.name, amount.quantity);
    }

    println!("Score = {:?}", max_score);
}
