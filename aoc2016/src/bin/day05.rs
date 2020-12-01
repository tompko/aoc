extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let input = "reyedfim";

    let mut hasher = Md5::new();
    let key = input.as_bytes();

    let mut pass1 = String::new();
    let mut pass2 = [None; 8];

    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(format!("{}", i).as_bytes());

        let mut output = [0;16];
        hasher.result(&mut output);

        let five = output[0] as u32 + output[1] as u32 + (output[2] >> 4) as u32;
        if five == 0 {
            let index = (output[2] & 0xf) as usize;
            let ch = format!("{:x}", output[3] >> 4).chars().next().unwrap();

            if pass1.len() < 8 {
                pass1 += &format!("{:x}", index);
            }
            if index < pass2.len() && pass2[index].is_none() {
                pass2[index] = Some(ch);
            }
        }

        if pass1.len() == 8 && pass2.iter().all(|p| p.is_some()) {
            break;
        }

        hasher.reset();
    }

    let pass2: String = pass2.iter().map(|p| p.unwrap()).collect();

    println!("1: {}", pass1);
    println!("2: {}", pass2);
}
