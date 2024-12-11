use std::io::Read;
use std::fs::File;

fn main() {
    let mut input_file = File::open("input.txt").unwrap();

    let mut content = String::new();
    input_file.read_to_string(&mut content).unwrap();

    let mut safe_reports = 0;

    content.split("\n").for_each(|l| {
        if l.trim().len() == 0{
            return;
        }
        println!("The report {}", l);
        let levels: Vec<i32> = l.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        let mut prev = None;
        let mut is_asc = None;

        let mut valid = true;

        for l in levels {
            println!("The level {}", l);
            if let Some(p) = prev {
                let diff: i32 = l - p;
                println!("l={}, p={}, diff={}", l, p, diff);
                if let None = is_asc {
                    is_asc = Some(diff > 0)
                }

                if diff == 0 {
                    valid = false;
                    break;
                }

                if is_asc.unwrap() && diff < 0 {
                    valid = false;
                    break;
                }

                if !is_asc.unwrap() && diff > 0 {
                    valid = false;
                    break;
                }

                if diff.abs() > 3 {
                    valid = false;
                    break;
                }

            }
            prev = Some(l);
        }

        if valid {
            safe_reports += 1;
            println!("Valid Report");
        } else {
            println!("invalid report");
        }
    });

    println!("The no of safe reports {}", safe_reports);
}
