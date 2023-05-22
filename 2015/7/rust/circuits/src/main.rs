mod io;
fn main() {
    for line in io::lines() {
        println!("{}", line);
    }
}
