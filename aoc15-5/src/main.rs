use std::io;
use std::io::prelude::*;

fn check_part1(str: &String)->bool {
    let bytes = str.as_bytes();
    let mut double = false;
    let mut vowels = 0;
    let mut bad = false;

    for i in 0..(str.len()) {
        let ch = bytes[i] as char;

        if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
            vowels+=1;
        }

        if i != str.len() - 1 {
            let ch1 = bytes[i+1] as char;

            if ch == ch1 {
                double = true;
            }

            if (ch == 'a' && ch1 == 'b') ||
                  (ch == 'c' && ch1 == 'd') ||
                  (ch == 'p' && ch1 == 'q') ||
                  (ch == 'x' && ch1 == 'y') {
                bad = true;
                break;
            }
        }
    }
    if !bad && double && vowels >= 3 {
        return true;
    } else {
        return false;
    }
}

fn check_part2(str: &String)->bool {
    let bytes = str.as_bytes();
    let mut repeat = false;
    let mut double = false;

    for i in 0..(str.len() - 2) {
        let ch = bytes[i] as char;
        let ch1 = bytes[i+1] as char;
        let ch2 = bytes[i+2] as char;

        if ch == ch2 {
            repeat = true;
        }

        if i != str.len() - 3 {
            for j in i+2..str.len()-1 {
                let m1 = bytes[j] as char;
                let m2 = bytes[j+1] as char;

                if ch == m1 && ch1 == m2 {
                    double = true;
                    break;
                }
            }
        }
    }
    if double && repeat {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let stdin = io::stdin();

    let mut good_strings1 = 0;
    let mut good_strings2 = 0;

    for line in stdin.lock().lines() {
        let ln = line.unwrap();
        if check_part1(&ln) {
            good_strings1+=1;
        }
        if check_part2(&ln) {
            good_strings2+=1;
        }
    }
    println!("Part 1: {} good strings.", good_strings1);
    println!("Part 2: {} good strings.", good_strings2);
}
