fn main() {
    let weights = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
    let total_weight = weights.iter().sum::<usize>();

    assert!(total_weight % 3 == 0);
    println!("total_weight = {total_weight}");
}
