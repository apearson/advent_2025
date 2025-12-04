use std::fs;

pub fn day3() {
  let input = fs::read_to_string("./src/day3/input").expect("Failed to read input file");
  
  let sum: i64 = input.lines().map(find_max_battery).sum();
  
  println!("Sum of best batterys: {}", sum)
}

fn find_max_battery(line: &str) -> i64{
  
  let numbers = line.split("").filter_map(|s| s.parse::<u8>().ok()).collect::<Vec<_>>();
  let numbers_len = numbers.len();
  let last_numbers_index = numbers_len - 1;
  
  // println!("Testing: {} {} last {} on {}", numbers_len, last_numbers_index, numbers[last_numbers_index],  line);
  
  // Stage 1 (find max tens int) 
  let mut max = (0, 0);
  
  // Loop through 0 to len-2 
  // (exclude the last number as it can only be used for the ones place)
  for i in 0..(last_numbers_index) {
    let current = numbers[i];
    
    // Found a larger umber
    if current > max.1 {
      max = (i, current);
    }
  }
  
  // println!("Stage 2: {} {} on {}", max.0, last_numbers_index - 1, line);

  
  // Stage 2 (find max ones int)
  let mut ones = (last_numbers_index, numbers[last_numbers_index]);
  
  // Loop through our max found (+1) to the second to last place 
  // (exclude the last number as it's our default)
  for i in (max.0 + 1)..last_numbers_index {
    let current = numbers[i];
    
    if current > ones.1 {
      ones = (i, current);
    }
  }
  
  let total: i64 = ((max.1 * 10) + ones.1).into();
  
  // Debug print 
  println!("Best battery is {} [{},{}] from {}", total, max.1, ones.1, line);
  
  return total.into();
}