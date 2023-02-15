use std::fs;

fn main() {
    let fileString = fs::read_to_string("./assets/DayOne-INPUT")
        .expect("Should have been able to read the file");
    let values: Vec<&str> = fileString.split("\n").collect();

    let mut elfs: Vec<i32> = vec![];
    let mut temp: i32 = 0;
    for i in values {
        if i == "" {
            elfs.push(temp);
            temp = 0;
            continue;
        }
        let i: i32 = i.parse().unwrap();
        temp = temp + i;
    }

    let mut result: (i32, i32) = (0, 0);

    let i: i32 = 0;

    while i < elfs.len() as i32 {
        if result.0 < elfs[i] {
            result = (elfs[i], i);
        } 
    }

    println!("{}", result.1);
}
