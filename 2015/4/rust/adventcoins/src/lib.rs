use md5;
use rayon::prelude::*;

pub fn nonce(msg: &str, prefix: &str) -> Option<u64> {
    (1u64..u64::MAX).into_iter().find(|&i| {
        let word = std::format!("{}{}", msg, i);
        let digest = md5::compute(word);
        std::format!("{:x}", digest).starts_with(prefix)
    })
}

pub fn nonce_par(msg: &str, prefix: &str) -> Option<u64> {
    (1u64..u64::MAX).into_par_iter().find_first(|&i| {
        let word = std::format!("{}{}", msg, i);
        let digest = md5::compute(word);
        std::format!("{:x}", digest).starts_with(prefix)
    })
}
