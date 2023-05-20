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

    for op in vec![
        "toggle 461,550 through 564,900",
        "turn off 370,39 through 425,839",
        "turn off 464,858 through 833,915",
        "turn off 812,389 through 865,874",
        "turn on 599,989 through 806,993",
        "turn on 376,415 through 768,548",
        "turn on 606,361 through 892,600",
        "turn off 448,208 through 645,684",
        "toggle 50,472 through 452,788",
        "toggle 205,417 through 703,826",
        "toggle 533,331 through 906,873",
        "toggle 857,493 through 989,970",
        "turn off 631,950 through 894,975",
        "turn off 387,19 through 720,700",
        "turn off 511,843 through 581,945",
        "toggle 514,557 through 662,883",
        "turn off 269,809 through 876,847",
        "turn off 149,517 through 716,777",
        "turn off 994,939 through 998,988",
        "toggle 467,662 through 555,957",
        "turn on 952,417 through 954,845",
    ]
    .into_iter()
    .map(str::parse::<Op>)
    {
        println!("and then: {:?}", op);
    }
}
