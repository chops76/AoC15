use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let mut good_strings = 0;

    for line in stdin.lock().lines() {
        let ln = line.unwrap();
        let bytes = ln.as_bytes();
        let mut double = false;
        let mut vowels = 0;
        let mut bad = false;
        for i in 0..(ln.len()-1) {
            let ch = bytes[i] as char;
            let ch1 = bytes[i+1] as char;

            if i != ln.len() - 1 && ch == ch1 {
                double = true;
            }
            if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
                    vowels+=1;
                }
            if i != ln.len() - 1 && ((ch == 'a' && ch1 == 'b') ||
                                     (ch == 'c' && ch1 == 'd') ||
                                     (ch == 'p' && ch1 == 'q') ||
                                     (ch == 'x' && ch1 == 'y')) {
                bad = true;
                break;
            }
        }
        if !bad && double && vowels >= 3 {
            good_strings += 1;
        }
    }
    println!("{} good strings.", good_strings);
}
