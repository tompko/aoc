extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

const INPUT: &'static str = "bgvyzdsv";

fn search(prefix: &str, target: &str) -> u32 {
    let mut suffix = 0;
    let mut m = Md5::new();
    loop {
        let string = prefix.to_string() + &suffix.to_string();
        m.reset();
        m.input_str(&string);
        if m.result_str().starts_with(target) {
            return suffix;
        }
        suffix += 1;
    }

}

fn main() {
    println!("{}", search(INPUT, "00000"));
    println!("{}", search(INPUT, "000000"));
}
