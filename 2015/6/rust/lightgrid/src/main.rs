use lightgrid::{GridPoint, Op};

fn main() {
    println!("hello, nom!");
    let point = "100,101".parse::<GridPoint>();
    println!("here is a point: {:?}", point);

    let op: Result<Op, _> = "toggle 100,101 through 200,202".parse();
    println!("here is an operation: {:?}", op);

    let ops: Vec<Result<Op, _>> = vec![
        "toggle 0,0 through 10,10",
        "toggle 10,10 through 0,0",
        "turn on 0,0 through 5,5",
        "turn off 3,4 through 7,5",
    ]
    .into_iter()
    .map(str::parse)
    .collect();

    for op in ops {
        println!("next op: {:?}", op);
    }
}
