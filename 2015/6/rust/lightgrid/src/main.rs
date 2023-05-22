mod io;

fn main() {
    env_logger::init();
    println!("{}", lightgrid::total_brightness(crate::io::lines()));
}
