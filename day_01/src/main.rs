use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Couldn't read the file");
    let mut numbers: Vec<i32> = Vec::new();
    for line in content.lines() {
        let mut output: String = String::new();
        // Find first number in line
        for ch in line.chars() {
            if ch.is_numeric() {
                output += &(ch.to_string());
                break;
            }   
        }
        // Find second number in line
        for ch in line.chars().rev() {
            if ch.is_numeric() {
                output += &(ch.to_string());
                break;
            }
        }
        let output : i32 = output.parse().unwrap();
        numbers.push(output);
    }
    dbg!(&numbers);
    let mut sum = 0;
    for num in numbers {
        sum += num;
    }
    println!("{}", sum);
}
