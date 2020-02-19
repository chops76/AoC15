use itertools::Itertools;

use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();

    let mut places = HashSet::<String>::new();
    let mut distances = HashMap::<(String, String), u32>::new();

    for line in stdin.lock().lines() {
        let ln = line.unwrap();
        let split = ln.split(' ');
        let vec: Vec<&str> = split.collect();
        let p1 = vec[0].to_string();
        let p2 = vec[2].to_string();
        places.insert(p1);
        places.insert(p2);

        let pair1 = (vec[0].to_string(), vec[2].to_string());
        let pair2 = (vec[2].to_string(), vec[0].to_string());

        distances.insert(pair1, vec[4].parse::<u32>().unwrap() );
        distances.insert(pair2, vec[4].parse::<u32>().unwrap() );
    }

    let perms = places.iter().permutations(8);
    let mut smallest = 10000;
    let mut longest = 0;
    for perm in perms {
        let mut total = 0;
        for i in 1..perm.len() {
            let p1 = perm[i-1].to_string();
            let p2 = perm[i].to_string();
            let tup = (p1, p2);
            total += distances[&tup];
        }
        if total < smallest {
            smallest = total;
        }
        if total > longest {
            longest = total;
        }
    }
    println!("Smallest distance: {}", smallest);
    println!("Longest distance: {}", longest)
}
