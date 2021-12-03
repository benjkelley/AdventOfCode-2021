use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").expect("Unable to open input");
    let f = BufReader::new(f);

    let mut hpos: i32 = 0;
    let mut dpos: i32 = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let splitstr: Vec<&str> = line.split(" ").collect();
        
        if splitstr[0].eq("forward") {
            hpos += splitstr[1].parse::<i32>().unwrap();
        }
        else if splitstr[0].eq("down") {
            dpos += splitstr[1].parse::<i32>().unwrap();
        }
        else if splitstr[0].eq("up") {
            dpos -= splitstr[1].parse::<i32>().unwrap();
        }

    }

    println!("hpos: {}", hpos);
    println!("dpos: {}", dpos);
    println!("answer: {}", hpos * dpos);

}
