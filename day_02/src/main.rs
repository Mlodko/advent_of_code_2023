use std::fmt;
use core::cmp::max;

struct Set {
    red : i32,
    green : i32,
    blue : i32,
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Set")
         .field("Red", &self.red)
         .field("Green", &self.green)
         .field("Blue", &self.blue)
         .finish()
    }
}

impl Set {
    fn parse_str(input: &str) -> Set {
        let contents = input.split(",");
        let mut set = Set::new(0, 0, 0);
        for phrase in contents {
            let parts : Vec<&str> = phrase.split_whitespace().collect();
            let number : i32 = parts[0].parse().unwrap();
            let color : &str = parts[1];
            match color {
                "red" => set.red += number,
                "green" => set.green += number,
                "blue" => set.blue += number,
                _ => continue
            }
        }
        set
        
    }

    fn get_power(&self) -> i32 {
        self.red * self.green * self.blue
    }
    
    fn new(red : i32, green : i32, blue: i32) -> Set {
        Set { red: red, green: green, blue: blue }
    }
}

fn main() {
    let max_set = Set {
        red: 12,
        green: 13,
        blue: 14
    };
    let input_string = std::fs::read_to_string("input.txt").expect("Couldn't read file");
    let mut game_id_sum = 0;
    let mut min_set_power_sum = 0;
    for game in input_string.lines() {
        // Read game ID
        let mut words : Vec<&str> = game.split_whitespace().collect();
        let game_id: i32 = words[1].strip_suffix(":").unwrap().parse().expect("Couldn't parse game_id to int");
        
        // Remove 1st and 2nd word
        words.remove(0);
        words.remove(0);
        
        // Merge vector back into string
        let words = words.join(" ");

        let mut set_vector : Vec<Set> = Vec::new();
        
        // Split into sets, merge into vector
        for set_string in words.split(";") {
            let set = Set::parse_str(set_string);
            set_vector.push(set);
        }

        let mut if_add : bool = true;

        // Check if drawn cubes are more than in max_set
        for set in &set_vector {
            // If it's possible
            if (set.red <= max_set.red) & (set.green <= max_set.green) & (set.blue <= max_set.blue) {
                if_add = true;
                continue;
            }
            else {
                if_add = false;
                break;
            }
        }
        if if_add
        {
            game_id_sum += game_id;
        }
        let mut min_set = Set::new(0,0,0);
        for set in &set_vector {
            min_set.red = max(min_set.red, set.red);
            min_set.green = max(min_set.green, set.green);
            min_set.blue = max(min_set.blue, set.blue);
        }
        min_set_power_sum += min_set.get_power()
    }
    dbg!(game_id_sum);
    dbg!(min_set_power_sum);

}
