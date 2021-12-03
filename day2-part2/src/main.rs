use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let f = BufReader::new(f);

    let mut hpos: i32 = 0;
    let mut dpos: i32 = 0;
    let mut aim: i32 = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let splitstr: Vec<&str> = line.split(" ").collect();
        
        if splitstr[0].eq("forward") {
            let change: i32 = splitstr[1].parse::<i32>().unwrap();
            hpos += change;
            dpos += aim * change;
        }
        else if splitstr[0].eq("down") {
            let change: i32 = splitstr[1].parse::<i32>().unwrap();
         //   dpos += change;
            aim += change;
        }
        else if splitstr[0].eq("up") {
            let change: i32 = splitstr[1].parse::<i32>().unwrap();
           // dpos -= change;
            aim -= change;
        }

    }

    println!("hpos: {}", hpos);
    println!("dpos: {}", dpos);
    println!("answer: {}", hpos * dpos);

}
