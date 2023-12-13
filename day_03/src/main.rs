use std::fmt::Debug;

struct FoundNumber {
    value: usize,
    line_index: usize,
    start_index: usize,
    end_index: usize
}

impl Debug for FoundNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FoundNumber")
            .field("value", &self.value)
            .field("line_index", &self.line_index)
            .field("start_index:", &self.start_index)
            .field("end_index", &self.end_index)
            .finish()
    }
}

impl FoundNumber {
    fn new(value: usize, line_index: usize, start_index: usize, end_index: usize) -> FoundNumber {
        FoundNumber {
            value,
            line_index,
            start_index,
            end_index
        }
    }
}

fn main() {
    let input_file = std::fs::read_to_string("input.txt").expect("Couldn't read file");
    let input_lines = input_file.lines();

    // Finding all numbers 
    let mut found_numbers : Vec<FoundNumber> = Vec::new();
    for line in input_lines.clone().enumerate() {
        let chars : Vec<char> = line.1.chars().collect();
        let mut start_index = 0;
        while start_index < chars.len() {
            println!("Checking ({}, {})", line.0, start_index);
            if !chars[start_index].is_alphanumeric() {
                start_index += 1;
                continue;
            }

            if chars[start_index].is_numeric() {
                let mut number = String::from(chars[start_index]);
                if start_index >= line.1.len() {
                    break;
                }
                let mut end_index = start_index + 1;
                loop {
                    if !chars[end_index].is_numeric() {
                        let number : usize = number.parse().unwrap();
                        let found_number = FoundNumber::new(number, line.0, start_index, end_index - 1);
                        start_index = end_index;
                        found_numbers.push(found_number);
                        break;
                    }
                    number.push(chars[end_index]);
                    end_index += 1;
                }
            }
        }
    }

    dbg!(&found_numbers);

    // Checking if there's a symbol around the numbers
    let mut sum_numbers = 0;
    let lines : Vec<&str> = input_lines.collect();
    let line_length = lines[0].len();
    'numberLoop: for num in found_numbers {
        let checked_start_index = {
            if num.start_index == 0 {
                0
            }
            else {
                num.start_index - 1
            }
        };
        let checked_end_index = {
            if num.end_index == line_length - 1 {
                line_length - 1
            }
            else {
                num.end_index + 1
            }
        };
        // The line above
        if num.line_index != 0 {
            let checked_string = &lines.get(num.line_index - 1).unwrap()[checked_start_index..checked_end_index];
            if !checked_string.chars().all(char::is_alphanumeric) & !checked_string. {
                sum_numbers += num.value;
                continue 'numberLoop;
            }
        }

        // The same line
        let checked_string = &lines.get(num.line_index).unwrap()[checked_start_index..checked_end_index];
        if !checked_string.chars().all(char::is_alphanumeric) & !checked_string.starts_with('.') & !checked_string.ends_with('.') {
            sum_numbers += num.value;
            continue 'numberLoop; 
        }

        // The line below
        if num.line_index != lines.len() - 1 {
            let checked_string = &lines.get(num.line_index + 1).unwrap()[checked_start_index..checked_end_index];
            if !checked_string.chars().all(char::is_alphanumeric) {
                sum_numbers += num.value;
                continue 'numberLoop;
            }
        }
        dbg!(num.value);
    }
    dbg!(sum_numbers);
    
}
