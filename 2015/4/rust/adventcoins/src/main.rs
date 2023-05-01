use md5;

fn nonce(msg: &str) -> Option<u64> {
    for i in 0u64.. {
        let word = std::format!("{}{}", msg, i);
        let digest = md5::compute(word);
        if std::format!("{:x}", digest).starts_with("00000") {
            return Some(i)
        }
    }
    None
}

fn main() {
    let value = nonce("bgvyzdsv");
    println!("value = {:?}", value);
}
