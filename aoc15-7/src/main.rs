use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

struct Gate {
    inp1: String,
    inp2: String,
    gate: String
}

impl Gate {
    fn new(i1: &str, i2: &str, g: &str) -> Gate {
        Gate { inp1: i1.to_string(), inp2: i2.to_string(), gate: g.to_string() }
    }
}

fn insert_gate(gatestr: String, hm: &mut HashMap<String, Gate>)
{
    let split = gatestr.split(' ');
    let vec: Vec<&str> = split.collect();
    if vec[1] == "->" {
        hm.insert(vec[2].to_string(), Gate::new(vec[0], "N/A", "eq"));
    } else if vec[0] == "NOT" {
        hm.insert(vec[3].to_string(), Gate::new(vec[1], "N/A", "NOT"));
    } else {
        hm.insert(vec[4].to_string(), Gate::new(vec[0], vec[2], vec[1]));
    }
}

fn get_params(param1: &String, param2: &String, p1val: &mut u16, p2val: &mut u16,
    values: &HashMap<String, u16>) -> bool {
    let parsed1 = param1.parse::<u16>();
    if parsed1.is_err() {
        if values.contains_key(param1) {
            *p1val = values[param1];
        } else {
            return false;
        }
    } else {
        *p1val = parsed1.unwrap();
    }
    if param2 == "N/A" {
        return true;
    }
    let parsed2 = param2.parse::<u16>();
    if parsed2.is_err() {
        if values.contains_key(param2) {
            *p2val = values[param2];
        } else {
            return false;
        }
    } else {
        *p2val = parsed2.unwrap();
    }
    true
}

fn main() {
    let part2 = true;

    let stdin = io::stdin();

    let mut values = HashMap::<String, u16>::new();
    let mut gates = HashMap::new();

    for line in stdin.lock().lines() {
        let ln = line.unwrap();
        insert_gate(ln, &mut gates);
    }

    if part2 {
        values.insert("b".to_string(), 46065);
    }
    let mut found = false;
    while !found {
        for (output, gatecmd) in &gates {
            if !values.contains_key(output) {
                let mut val1:u16 = 0;
                let mut val2:u16 = 0;
                if get_params(&gatecmd.inp1, &gatecmd.inp2, &mut val1, &mut val2, &values) {
                    if gatecmd.gate == "eq" {
                        values.insert(output.to_string(), val1);
                    } else if gatecmd.gate == "LSHIFT" {
                        let answer = val1 << val2;
                        values.insert(output.to_string(), answer);
                    } else if gatecmd.gate == "RSHIFT" {
                        let answer = val1 >> val2;
                        values.insert(output.to_string(), answer);
                    } else if gatecmd.gate == "NOT" {
                        let answer = !val1;
                        values.insert(output.to_string(), answer);
                    } else if gatecmd.gate == "OR" {
                        let answer = val1 | val2;
                        values.insert(output.to_string(), answer);
                    } else if gatecmd.gate == "AND" {
                        let answer = val1 & val2;
                        values.insert(output.to_string(), answer);
                    }
                    if output == "a" {
                        println!("Found a: {}", values[output]);
                        found = true;
                        break;
                    }
                }
            }
        }
    }
}
