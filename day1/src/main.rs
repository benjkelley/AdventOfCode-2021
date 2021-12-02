use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input").expect("Unable to open input");
    let f = BufReader::new(f);

    let mut increased_count = 0;
    let mut last_num = 0;
    // let mut vals: Vec<i32> = Vec::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
    //    vals.push(line.parse::<i32>().unwrap());
        let line_as_int = line.parse::<i32>().unwrap();
        if line_as_int > last_num {
            increased_count += 1;
        }
        last_num = line_as_int;

        increased_count -= 1;       // decrement by 1 to skip the first case
    }
    println!("Answer: {}", increased_count);
}
