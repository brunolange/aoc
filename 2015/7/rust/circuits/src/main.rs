use circuits::run;

mod io;

fn main() {
    let wire_map = run(io::lines());
    println!("{:?}", wire_map);
}
