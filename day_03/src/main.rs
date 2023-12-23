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
        dbg!(line.0);
        let chars : Vec<char> = line.1.chars().collect();
        let mut start_index = 0;
        'search_loop: while start_index < line.1.len() {
            if !chars.get(start_index).unwrap().is_digit(10) {
                start_index += 1;
                continue;
            }
            let mut current_value : usize = 0;
            for end_index in start_index..line.1.len() {
                let checked_substring = line.1.get(start_index..=end_index).unwrap();
                let parsed_substring = checked_substring.parse::<usize>();
                match parsed_substring {
                    Ok(num) => {
                        current_value = num;
                    }
                    Err(_) =>{
                        let new_number = FoundNumber::new(current_value, line.0, start_index, end_index - 1);
                        dbg!(&new_number);
                        found_numbers.push(new_number);
                        start_index = end_index + 1;
                        continue 'search_loop;
                    }
                }
            }
            if current_value != 0 {
                let new_number = FoundNumber::new(current_value, line.0, start_index, line.1.len() - 1);
                dbg!(&new_number);
                found_numbers.push(new_number);
                start_index = line.1.len();
            }
        }
    }

    dbg!(&found_numbers);

    // Checking if there's a symbol around the numbers
    let mut sum_numbers = 0;
    let lines : Vec<&str> = input_lines.collect();
    'numberLoop: for num in found_numbers {
        'above_line: {
            let checked_line = lines.get(sub_or_zero(num.line_index, 1));
            match checked_line {
                None => break 'above_line, // No line above, you can skip this check
                Some(s) => {
                    let chars : Vec<char> = s.chars().collect();
                    for i in sub_or_zero(num.start_index, 1) ..= num.end_index + 1 {
                        if let Some(c) = chars.get(i) {
                            if is_symbol(*c) {
                                println!("Adding {} from above line", num.value);
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
        if let Some(c) = chars.get(sub_or_zero(num.start_index, 1)) {
            if is_symbol(*c) {
                println!("Adding {} from same line", num.value);
                sum_numbers += num.value;
                continue 'numberLoop;
            }
        }
        if let Some(c) = chars.get(num.end_index + 1) {
            if is_symbol(*c) {
                println!("Adding {} from same line", num.value);
                sum_numbers += num.value;
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

            for i in sub_or_zero(num.start_index, 1) ..= num.end_index + 1 {
                if let Some(c) = chars.get(i) {
                    if is_symbol(*c) {
                        println!("Adding {} from below line", num.value);
                        sum_numbers += num.value;
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
fn sub_or_zero(a: usize, b : usize) -> usize {
    if let Some(out) = a.checked_sub(b) {
        return out;
    }
    0
}