use std::fs;

fn main() {
    let file_string = fs::read_to_string("./assets/input") // the path to the .txt file of the
        // input
        .expect("Should have been able to read the file");
    let values: Vec<&str> = file_string.split("\n").collect();

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

    let mut result = 0;

    let mut i: usize = 0;

    while i < elfs.len() {
        if result < elfs[i] {
            result = elfs[i];
        }
        i += 1;
    }
    println!("{}", result);
}
