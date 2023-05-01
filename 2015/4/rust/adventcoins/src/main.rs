use md5;

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
    let value = nonce("bgvyzdsv");
    println!("value = {:?}", value);
}
