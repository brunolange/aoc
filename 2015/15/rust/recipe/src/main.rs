use recipe::n_multichoose_k;

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

#[derive(Debug)]
struct Amount<'a> {
    quantity: usize,
    ingredient: &'a Ingredient,
}

fn score(amounts: &[Amount]) -> usize {
    amounts
        .iter()
        .map(|a| {
            let ing = a.ingredient;
            let q = a.quantity;
            [ing.capacity, ing.durability, ing.flavor, ing.texture].map(|v| v * q as i64)
        })
        .reduce(|acc, curr| {
            acc.into_iter()
                .zip(curr)
                .map(|(a, b)| a + b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .unwrap()
        .into_iter()
        .map(|v| std::cmp::max(0, v) as usize)
        // .fold(1, |acc, curr| acc * curr) clippy FTW!
        .product()
}

fn main() {
    let ingredients = vec![
        Ingredient {
            name: "Butterscotch".to_string(),
            capacity: -1,
            durability: -2,
            flavor: 6,
            texture: 3,
            calories: 8,
        },
        Ingredient {
            name: "Cinnamon".to_string(),
            capacity: 2,
            durability: 3,
            flavor: -2,
            texture: -1,
            calories: 3,
        },
    ];
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
        .max_by_key(|(score, _amounts)| *score)
        .unwrap();

    for amount in amounts {
        println!("{}: {}", amount.ingredient.name, amount.quantity);
    }

    println!("Score = {:?}", max_score);
}
