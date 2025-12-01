use std::fs;

fn main() {
    day1();
    
    // debug();

}

fn debug() {
    let mut dial_position = 44;
    
    dial_position += 56;
    
    dial_position %= 100;
    
    println!("Testing {}",  dial_position);
}

fn day1() {
    let mut dial_position = 50;
    
    let input = fs::read_to_string("./src/day1/input").expect("Failed to read input file");
    
    let mut total_zeroes = 0;
    
    for line in input.lines() {
        // Get first char 
        let direction = line.chars().next().unwrap();
        
        // Get number
        let number = line[1..].trim().parse::<i32>().unwrap();
        
        let old_position = dial_position;
        
        match direction {
            'R' => {
                dial_position += number;
                
                if dial_position >= 100 { 
                    total_zeroes += (dial_position - 100) / 100 ; // Part 2
                    dial_position %= 100;
                }
            },
            'L' => {
                dial_position -= number;
                
                if dial_position <= 0 {
                    println!("Undeferflow: {}, Total Zeros Added: {}", dial_position, ((-dial_position) / 100) + 1);
                    total_zeroes += ((-dial_position) / 100) + 1; // Part 2
                    
                    dial_position += (100 * ((-dial_position / 100) + 1)) as i32;
                    dial_position %= 100;
                }
            },
            _ => panic!("Invalid direction"),
        }
        
        println!("Direction: {}, Number: {}, Old Position: {}, New Position: {}, Zero Total: {}", direction, number, old_position, dial_position, total_zeroes);
                
        if dial_position < 0 || dial_position >= 100 {
            panic!("Dial position out of bounds: {}", dial_position);
        }
        

    }
    
    println!("Total zeroes (password): {}", total_zeroes);
}