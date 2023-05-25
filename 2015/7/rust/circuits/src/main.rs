use circuits::run;

mod io;

fn main() {
    let wire_map = run(io::lines());
    let wire = io::wire();
    match wire {
        Ok(w) => println!("{:?}", wire_map.unwrap().get(&w)),
        _ => println!("{:?}", wire_map),
    }
}
