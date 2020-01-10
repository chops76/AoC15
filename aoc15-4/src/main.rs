extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let part1 = false;

    let my_key = "ckczppom";
    let mut md5calc = Md5::new();

    let mut prefix = "00000";
    if !part1 {
        prefix = "000000";
    }

    for i in 0..std::u64::MAX {
        md5calc.input(my_key.as_bytes());
        md5calc.input(i.to_string().as_bytes());

        let hash = md5calc.result_str();
        if hash.starts_with(prefix) {
            println!("Found at {} - MD5 is {}", i, hash);
            break;
        }
        
        md5calc.reset();
    }
}
