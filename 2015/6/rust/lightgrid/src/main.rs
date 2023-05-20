use lightgrid::GridPoint;

fn main() {
    println!("hello, nom!");
    let point = "100,101".parse::<GridPoint>();
    println!("here is a point: {:?}", point);
}
