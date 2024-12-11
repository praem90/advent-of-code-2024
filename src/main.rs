use std::env::args;
use std::io::Read;
use std::fs::File;

fn main() {
    let mut input_file = File::open("input.txt").unwrap();

    let mut content = String::new();
    input_file.read_to_string(&mut content).unwrap();

    let mut loc_a = vec![];
    let mut loc_b = vec![];

    content.split("\n").for_each(|l| {
        let mut line = l.split_whitespace();

        if let Some(a) = line.next() {
            loc_a.push(a.parse::<i32>().unwrap())
        }

        if let Some(b) = line.next() {
            loc_b.push(b.parse::<i32>().unwrap())
        }
    });

    let mut length: i32 = 0;
    let args_count = args().len();

    if args_count == 1 {
        loc_a.sort();
        loc_b.sort();

        for i in 0..loc_a.len() {
            let diff = loc_a.get(i).unwrap() - loc_b.get(i).unwrap();
            length = length + diff.abs();

            println!("l1={}, l2={}, diff={}, length={}",
                loc_a.get(i).unwrap(),
                loc_b.get(i).unwrap(),
                diff.abs(),
                length);
        }


    } else {
        for i in 0..loc_a.len() {
            let value = loc_a.get(i).unwrap();
            let n = loc_b.iter().filter(|&v| v == value).count() as i32;

            length = length + (value * n);
        }
    }

    println!("The second length is {}", length);
}
