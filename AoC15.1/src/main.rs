use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Couldn't read input");

    let mut count = 0;
    let mut i1 = input.chars();

    let mut been_neg = false;
    let mut first_neg = 0;
    let mut pos = 1;
   
    while let Some(ch) = i1.next() {
        if ch == '(' {
            count+=1;
        } else if ch == ')' {
            count-=1;
        }
        if count < 0 && been_neg == false {
            been_neg = true;
            first_neg = pos;
        }
        pos += 1;
    }
    println!("Final floor: {}", count);
    println!("First below ground position: {}", first_neg)
}
