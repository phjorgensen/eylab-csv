mod arguments;

use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let file_path_string =
        arguments::nth(1, "Missing the filepath to the file that is to be parsed.");
    let file_path = Path::new(&file_path_string);

    let new_file_path_string = arguments::nth(
        2,
        "Missing the filepath to where the new file will be created.",
    );
    let new_file_path = Path::new(&new_file_path_string);

    let skip_argument = arguments::get_value_from("skip", "No skip");
    let skips = format_skip(&skip_argument);
    println!("{:?}", skips);

    // Open the file to parse and add it to a buffer reader
    let file = fs::File::open(&file_path).expect("Could not find the existing file");
    let mut reader = io::BufReader::new(file);

    // Create the new file
    let mut new_file = fs::File::create(&new_file_path).expect("Could not create the new file");

    let mut line_number = 0;

    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }

        line_number += 1;

        let mut new_line = do_skips(&line, &skips);
        if !new_line.contains("\n") {
            new_line = new_line + "\n";
        }

        match new_file.write_all(new_line.as_bytes()) {
            Ok(_) => println!("Wrote line {} to {}", line_number, new_file_path.display()),
            Err(why) => println!("Could not write to {}: {}", new_file_path.display(), why),
        };
    }
}

fn do_skips(line: &String, skips: &Vec<(usize, usize)>) -> String {
    let line_parts: Vec<&str> = line.split(",").collect();
    let mut new_line_parts: Vec<&str> = vec![];

    for i in 0..line_parts.len() {
        let mut do_skip = false;

        for skip in skips {
            if do_skip {
                continue;
            }

            if (i + 1) >= (skip.0) && (i + 1) <= (skip.1) {
                do_skip = true;
            }
        }

        if do_skip {
            continue;
        }

        new_line_parts.push(line_parts[i]);
    }

    return new_line_parts.join(",");
}

fn format_skip(skip_argument: &str) -> Vec<(usize, usize)> {
    // Collection,Single,Less than last,Goes beyond max
    // 1-3,6,5,8-7,11-11,20-100

    let mut skip_groups: Vec<(usize, usize)> = skip_argument
        .split(",")
        .map(|group| {
            let mut split_groups: Vec<_> = group.split("-").collect();

            if split_groups.len() < 2 {
                split_groups.push(split_groups[0]);
            }

            let first_digit: usize = split_groups
                .get(0)
                .unwrap()
                .parse::<usize>()
                .expect("Could not find a number to skip");

            let second_digit: usize = split_groups
                .get(1)
                .unwrap()
                .parse::<usize>()
                .expect("Could not find a number to skip");

            if first_digit < second_digit {
                return (first_digit, second_digit);
            } else {
                return (second_digit, first_digit);
            }
        })
        .collect();

    skip_groups.sort_by(|a, b| a.0.cmp(&b.0));

    return skip_groups;
}
