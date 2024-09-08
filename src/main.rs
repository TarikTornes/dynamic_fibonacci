use std::io;

fn read_input() -> String {

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input.trim().to_string()
}

fn process_input(s: &str) -> Option<u64> {
    
    match s.parse::<u64>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

/*
fn compute_fib1(n: u64) -> u64{
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut fst = 0;
    let mut snd = 1;
    let mut res = 0;
    let mut x:u64 = 2;
    
    while x <= n {
        res = fst + snd;
        fst = snd;
        snd = res;
        x += 1;
    }
    res
}
*/

fn compute_fib2(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let (mut fst, mut snd) = (0,1);

    for _ in 1..n {
        (fst, snd) = (snd, fst + snd);
    }

    snd
}



fn main() {

    match process_input(&read_input()) {
        Some(val) => println!("Fib: {}", compute_fib2(val)),
        None => println!("Invalid input!"),
    }

}
