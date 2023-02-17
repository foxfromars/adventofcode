use std::fs;

fn main() {
    // A = ROCK
    // B = PAPER
    // C = SCISSORS

    // X = ROCK
    // Y = PAPER
    // Z = SCISSORS

    fn result(play1: char, play2: char) -> i32 {
        let mut score: i32 = 0;
        if play2 == 'X' {
            score += 1
        }

        if play2 == 'Y' {
            score += 2
        }

        if play2 == 'Z' {
            score += 3
        }

        if play1 == 'A' {
            if play2 == 'X' {
                score += 3 as i32;
            } else if play2 == 'Y' {
                score += 6 as i32;
            } else {
                score += 0 as i32;
            }
        } else if play1 == 'B' {
            if play2 == 'X' {
                score += 0 as i32;
            } else if play2 == 'Y' {
                score += 3 as i32;
            } else {
                score += 6 as i32;
            }
        } else {
            if play2 == 'X' {
                score += 6 as i32;
            } else if play2 == 'Y' {
                score += 0 as i32;
            } else {
                score += 3 as i32;
            }
        }
        return score;
    }

    fn process_data(path: &str) -> Vec<String> {
        let file_string = fs::read_to_string(path).expect("file should be read by the function");
        // let plays: Vec<String> = file_string.split("\n").map(|s| s.to_owned()).collect();
        let temp_plays: Vec<&str> = file_string.split("\n").collect();
        let mut plays: Vec<String> = vec![];

        for i in temp_plays {
            plays.push(i.split(" ").map(|s| s.to_owned()).collect());
        }
        return plays;
    }

    let plays: Vec<String> = process_data("./src/assets/input");
    let mut score: i32 = 0;

    for i in plays {
        //exception
        if i == "" {
            break;
        }

        let k: i32 = result(i.chars().nth(0).unwrap(), i.chars().nth(1).unwrap());
        score += k;
    }
    println!("{}", score);
}
