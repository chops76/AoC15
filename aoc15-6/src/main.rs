use std::io;
use std::io::prelude::*;

fn points(pointstr: &str)->(u32, u32) {
    let split = pointstr.split(',');
    let vec: Vec<&str> = split.collect();
    let x: u32 = vec[0].parse()
        .expect("Parse error");
    let y: u32 = vec[1].parse()
        .expect("Parse error");
    return (x, y);
}

fn main() {
    let stdin = io::stdin();

    let mut grid = vec![vec![false; 1000];1000];
    let mut grid2 = vec![vec![0; 1000]; 1000];

    for line in stdin.lock().lines() {
        let ln = line.unwrap();
        let split = ln.split(' ');
        let vec: Vec<&str> = split.collect();

        let mut toggle = false;
        let mut new_state = false;
        let x_min: usize;
        let x_max: usize;
        let y_min: usize;
        let y_max: usize;

        if vec[0] == "toggle" {
            let (x1, y1) = points(vec[1]);
            let (x2, y2) = points(vec[3]);
            toggle = true;
            x_min = x1 as usize;
            x_max = x2 as usize;
            y_min = y1 as usize;
            y_max = y2 as usize;
        } else if vec[1] == "on" {
            let (x1, y1) = points(vec[2]);
            let (x2, y2) = points(vec[4]);
            new_state = true;
            x_min = x1 as usize;
            x_max = x2 as usize;
            y_min = y1 as usize;
            y_max = y2 as usize;
        } else {
            let (x1, y1) = points(vec[2]);
            let (x2, y2) = points(vec[4]);
            x_min = x1 as usize;
            x_max = x2 as usize;
            y_min = y1 as usize;
            y_max = y2 as usize;
        }
        for i in x_min..x_max+1 {
            for j in y_min..y_max+1 {
                if toggle {
                    grid[i][j] = !grid[i][j];
                    grid2[i][j] += 2;
                } else if new_state {
                    grid[i][j] = true;
                    grid2[i][j] += 1;
                } else {
                    grid[i][j] = false;
                    if grid2[i][j] != 0 {
                        grid2[i][j] -= 1;
                    }
                }
            }
        }
    }
    let mut count = 0;
    let mut brightness = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if grid[i][j] {
                count += 1;
            }
            brightness += grid2[i][j];
        }
    }
    println!("Total number on (Part 1): {}", count);
    println!("Total brightness (Part 2): {}", brightness);
}
