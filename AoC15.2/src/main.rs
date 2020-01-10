use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let mut total_paper = 0;
    let mut total_ribbon = 0;
    for line in stdin.lock().lines() {
        let ln = line.unwrap();
        let mut dims = Vec::new();
        for dim in ln.split("x") {
            let dim: u32 = dim.parse()
                .expect("Parse error");
            dims.push(dim);
        }
        dims.sort();
        total_paper += 3 * dims[0] * dims[1];
        total_paper += 2 * dims[0] * dims[2];
        total_paper += 2 * dims[1] * dims[2];

        total_ribbon += 2 * (dims[0] + dims[1]);
        total_ribbon += dims[0] * dims[1] * dims[2];
    }
    println!("Total paper needed: {}", total_paper);
    println!("Total ribbon needed: {}", total_ribbon);
}
