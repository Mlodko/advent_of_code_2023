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
    'numberLoop: for num in found_numbers {
        'above_line: {
            let checked_line = lines.get(num.line_index - 1);
            match checked_line {
                None => break 'above_line, // No line above, you can skip this check
                Some(s) => {
                    let chars : Vec<char> = s.chars().collect();
                    for i in num.start_index - 1 ..= num.end_index + 1 {
                        if let Some(c) = chars.get(i) {
                            if is_symbol(*c) {
                                sum_numbers += num.value;
                                continue 'numberLoop;
                            }
                        }
                        
                    }
                }
            }
        }

        // Same line
        let chars : Vec<char> = lines.get(num.line_index).unwrap().chars().collect();
        if let Some(c) = chars.get(num.start_index - 1) {
            if is_symbol(*c) {
                sum_numbers += 1;
                continue 'numberLoop;
            }
        }
        if let Some(c) = chars.get(num.end_index + 1) {
            if is_symbol(*c) {
                sum_numbers += 1;
                continue 'numberLoop;
            }
        }

        'below_line: {
            let chars: Vec<char>;
            if let Some(line) = lines.get(num.line_index + 1) {
                chars = line.chars().collect();
            }
            else {
                break 'below_line; // No below line, skip this check
            }

            for i in num.start_index - 1 ..= num.end_index + 1 {
                if let Some(c) = chars.get(i) {
                    if is_symbol(*c) {
                        sum_numbers += 1;
                        continue 'numberLoop;
                    }
                }
            }
        }

    }
    dbg!(sum_numbers);
    
}

fn is_symbol(c : char) -> bool {
    !c.is_alphanumeric() & (c != '.')
}