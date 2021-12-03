use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input").expect("Unable to open input");
    let f = BufReader::new(f);

    let mut increased_count = 0;
    let mut vals: Vec<i32> = Vec::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        vals.push(line.parse::<i32>().unwrap());
    }

    let mut first_triplet: Vec<i32> = Vec::new();
    let mut second_triplet: Vec<i32> = Vec::new();
    let mut first_trip_sum: i32 = 0;
    let mut second_trip_sum: i32 = 0;

    for val in vals.iter() {
        if first_triplet.len() < 3 {
            first_triplet.push(*val);
        }
        else if first_triplet.len() == 3 && second_triplet.len() < 3 {
            second_triplet.push(first_triplet[1]);
            second_triplet.push(first_triplet[2]);
            second_triplet.push(*val);
        }
        else {
            first_trip_sum = first_triplet.iter().sum();
            println!("First trip sum: {}\n", first_trip_sum);
            second_trip_sum = second_triplet.iter().sum();
            println!("Second trip sum: {}\n", second_trip_sum);
            
            if second_trip_sum > first_trip_sum {
                println!("2 > 1, increasing\n");
                increased_count += 1;
            }

            first_triplet = second_triplet.to_vec();
            second_triplet.remove(0);
            second_triplet.push(*val);
        }
    }
    if second_trip_sum > first_trip_sum {
        increased_count += 1;
    }

    println!("Answer: {}", increased_count);
}
