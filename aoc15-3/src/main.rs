use std::io;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Pos {
    x: i32,
    y: i32
}

fn main() {
    let part1 = false;

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Couldn't read input");

    let mut i1 = input.chars();

    let mut x_santa = 0;
    let mut y_santa = 0;

    let mut x_robot = 0;
    let mut y_robot = 0;

    let mut visited = HashSet::new();
    visited.insert(Pos {x: 0, y: 0});
   
    let mut count = 0;
    while let Some(ch) = i1.next() {
        let mut x_pos = &mut x_santa;
        let mut y_pos = &mut y_santa;
        if count % 2 != 0 && !part1
        {
            x_pos = &mut x_robot;
            y_pos = &mut y_robot;
        }
        if ch == '>' {
            *x_pos+=1;
        } else if ch == '<' {
            *x_pos-=1;
        } else if ch == '^' {
            *y_pos-=1;
        } else if ch == 'v' {
            *y_pos += 1;
        }
        visited.insert(Pos {x: *x_pos, y: *y_pos});
        count += 1;
    }
    println!("Houses Visisted: {}", visited.len());
}

