use std::fs;

pub fn day1() {
    let mut dial_position = 50;
    let mut total_zeroes = 0;
    
    let input = fs::read_to_string("./src/day1/input").expect("Failed to read input file");
    
    for line in input.lines() {
        // Get first char 
        let direction = line.chars().next().unwrap();        
        let number = line[1..].trim().parse::<i32>().unwrap();
        
        let old_position = dial_position;
        
        match direction {
            'R' => {
                dial_position += number;
                
                if dial_position >= 100 { 
                    let zeros_to_add = dial_position / 100;
                    
                    total_zeroes += zeros_to_add; // Part 2
                    
                    println!("Overflow: {}, Total Zeros Added: {}", dial_position, zeros_to_add);
                    
                    dial_position %= 100;
                }
            },
            'L' => {
                dial_position -= number;
                
                if dial_position <= 0 {
                    
                    let mut zeroes_to_add = (-dial_position) / 100;
                    
                    if old_position != 0 {
                        zeroes_to_add += 1;
                    }
                    
                    println!("Underflow: {}, Total Zeros Added: {}", dial_position, zeroes_to_add);
                    
                    total_zeroes += zeroes_to_add; // Part 2
                    
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