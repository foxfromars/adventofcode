use std::fs;

fn main() {
    println!("Hello, world!");

    let file_string = fs::read_to_string("./src/assets/input").expect("Could not read the file");

    let rucksacks: Vec<&str> = file_string.split("\n").collect();
    let mut duplicates: Vec<i32> = vec![];

    for line in &rucksacks {
        let mut found: bool = false;
        for i in 0..(line.len() / 2) {
            for k in (line.len() / 2)..line.len() {
                if line.chars().nth(i) == line.chars().nth(k) {
                    let caracter: char = line.chars().nth(i).clone().unwrap();
                    if caracter.is_lowercase() {
                        duplicates.push(caracter as i32 - 96);
                    } else {
                        duplicates.push(caracter as i32 - 27);
                    }
                    found = true;
                    break;
                }
            }
            if found == true {
                break;
            }
        }
    }

    let mut total: i32 = 0;

    for i in duplicates.iter() {
        println!("{}", i);
        total += i;
    }
    println!("{}", total);
    println!("{}", rucksacks.len());
    println!("{}", duplicates.len());
}
