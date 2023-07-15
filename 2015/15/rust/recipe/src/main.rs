use recipe::f;

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

struct Amount<'a> {
    quantity: usize,
    ingredient: &'a Ingredient,
}

fn score(amounts: &Vec<Amount>) -> usize {
    let xs = amounts
        .iter()
        .map(|a| {
            let ing = a.ingredient;
            let q = a.quantity;
            let v = [ing.capacity, ing.durability, ing.flavor, ing.texture].map(|v| v * q as i32);

            println!("v = {:?}", v);

            v
        })
        .reduce(|acc, curr| {
            acc.into_iter()
                .zip(curr)
                .map(|(a, b)| a + b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .unwrap();

    println!("xs = {:?}", xs);

    let ys = xs.into_iter().fold(1, |acc, curr| acc * curr);

    std::cmp::max(0, ys) as usize
}

fn main() {
    let ingredients = vec![
        Ingredient {
            capacity: -1,
            durability: -2,
            flavor: 6,
            texture: 3,
            calories: 8,
        },
        Ingredient {
            capacity: 2,
            durability: 3,
            flavor: -2,
            texture: -1,
            calories: 3,
        },
    ];

    println!(
        "{}",
        score(&vec![
            Amount {
                quantity: 44,
                ingredient: &ingredients[0],
            },
            Amount {
                quantity: 56,
                ingredient: &ingredients[1],
            }
        ])
    );

    println!("{:?}", f(4, 3));
}
