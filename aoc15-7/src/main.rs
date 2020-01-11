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

fn main() {
    let stdin = io::stdin();

    let mut values = HashMap::<String, i32>::new();
    let mut gates = HashMap::new();

    for line in stdin.lock().lines() {
        let ln = line.unwrap();
        insert_gate(ln, &mut gates);
    }

    loop {
        for (output, gatecmd) in &gates {
            if !values.contains_key(output) {
                if gatecmd.gate == "eq" {
                    let my_int = gatecmd.inp1.parse::<i32>();
                    if my_int.is_err() {
                        if values.contains_key(&gatecmd.inp1) {
                            values.insert(output.to_string(), values[&gatecmd.inp1]);
                            println!("eq, indirect");
                        }
                    } else {
                        values.insert(output.to_string(), my_int.unwrap());
                        println!("{}: {}({}, {})", output, gatecmd.gate, gatecmd.inp1, gatecmd.inp2);
                    }
                }
                //println!("{}: {}({}, {})", output, gatecmd.gate, gatecmd.inp1, gatecmd.inp2);
            }
        }
    }
}
