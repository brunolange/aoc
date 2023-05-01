use md5;
use std::io;

fn nonce(msg: &str) -> Option<u64> {
    (1u64..u64::MAX)
        .into_iter()
        .find(|&i| {
            let word = std::format!("{}{}", msg, i);
            let digest = md5::compute(word);
            std::format!("{:x}", digest).starts_with("000000")
        })
}

fn main() {
    let mut input = String::new();
    while let Ok(_) = io::stdin().read_line(&mut input) {
        let msg = input.trim();
        if msg.is_empty() {
            break;
        }
        let value = nonce(msg).unwrap();
        println!("value = {:?}", value);
        input.clear();
    }
}
