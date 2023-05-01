use md5;

fn nonce(msg: &str) -> u64 {
    let mut i = 0;
    loop {
        let word = std::format!("{}{}", msg, i);
        let digest = md5::compute(word);
        if std::format!("{:x}", digest).starts_with("00000") {
            break
        }
        i += 1;
    }
    i
}

fn main() {
    let value = nonce("bgvyzdsv");
    println!("value = {}", value);
}
