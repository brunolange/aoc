use circuits::run;

mod io;

use io::Output;

fn main() {
    env_logger::init();

    let signal_map = run(io::lines());
    let wire = io::wire();

    let output = signal_map.map(|wm| match wire {
        Ok(w) => wm.get(&w).map_or(Output::Error, |v| Output::SingleWire(*v)),
        Err(_) => Output::AllWires(wm),
    });

    println!("{:?}", output);
}
