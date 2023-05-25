use circuits::run;

mod io;

use io::Output;

fn main() {
    env_logger::init();

    let wire_map = run(io::lines());
    let wire = io::wire();

    let output = wire_map.map(|wm| match wire {
        Err(_) => Output::AllWires(wm),
        Ok(w) => wm.get(&w).map_or(Output::Error, |v| Output::SingleWire(*v)),
    });

    println!("{:?}", output);
}
