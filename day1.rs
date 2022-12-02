use std::io;
use std::io::BufRead;

fn main() {
    let (max, _) = io::stdin().lock().lines().fold((0,0), |(max, current_calories), input|{
        match (input.unwrap().as_str(), current_calories > max) {
            ("", true) => (current_calories, 0),
            ("", false) => (max, 0),
            (string, _) => (max, current_calories + string.parse::<i32>().unwrap())
        }
    });
    println!("most calories = {}", max);
}


