use std::io::BufRead;
use std::thread::sleep;
use std::time::Duration;

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn flash(s: &str, duration_in_millis: u64) {
    println!("{s}");
    sleep(Duration::from_millis(duration_in_millis));
    clear();
}

pub fn lines() -> impl Iterator<Item = String> {
    std::io::stdin().lock().lines().map_while(Result::ok)
}
