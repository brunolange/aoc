use circuits::run;

mod io;

fn main() {
    env_logger::init();
    let wire_map = run(io::lines());
    let wire = io::wire();
    let output = match wire {
        Ok(w) => match &wire_map {
            Some(wp) => wp.get(&w),
            _ => None,
        },
        _ => None,
    };
    println!("{:?}", output);
}
