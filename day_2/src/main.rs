use std::fs;
use std::vec::IntoIter;

fn main() {
    // A = ROCK
    // B = PAPER
    // C = SCISSORS

    // X = ROCK
    // Y = PAPER
    // Z = SCISSORS

    fn result(play1: char, play2: char) -> i32 {
        if play1 == 'A' {
            if play2 == 'X' {
                return 6 as i32;
            } else if play2 == 'Y' {
                return 3 as i32;
            } else {
                return 8 as i32;
            }
        }

        else if play1 == 'B' {
            if play2 == 'X' {
                return 8 as i32;
            } else if play2 == 'Y' {
                return 6 as i32;
            } else {
                return 3 as i32;
            }
        }

        else {
            if play2 == 'X' {
                return 3 as i32;
            } else if play2 == 'Y' {
                return 8 as i32;
            } else {
                return 6 as i32;
            }
        }
    }
   
    fn process_data (path: &str) -> Vec<String> {
        let file_string = fs::read_to_string(path)
            .expect("file should be read by the function");
        // let plays: Vec<String> = file_string.split("\n").map(|s| s.to_owned()).collect();
        let temp_plays: Vec<&str> = file_string.split("\n").collect();
        let mut plays: Vec<String> = vec![];
        
        for i in temp_plays {
            plays.push(i.split(" ").map(|s| s.to_owned()).collect());
        }
        return plays;
    }

    let plays: Vec<String> = process_data("./assets/input");
    for i in plays {
        println!("{}", i);
    }
}
