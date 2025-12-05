use std::{fs};

pub fn start() {
  let input = fs::read_to_string("./src/day5/input").expect("Failed to read input file");

  let mut fresh_nums: Vec<(i64, i64)> = Vec::new();
  let mut ingredients: Vec<i64> = Vec::new();
    
  // Parse Phase
  for line in input.lines() {
    
    if line.trim().is_empty() {
      println!("Empty Line");
      continue;
    }
    
    // Phase 1
    if line.contains('-') {
      let ranges: Vec<i64> = line.split('-').map(|n| n.parse().unwrap()).collect();
      
      println!("Found Range: {} to {}", ranges[0], ranges[1]);
      fresh_nums.push((ranges[0], ranges[1]));
      continue;
    }
    
    // Phase 2
    println!("Found ingredient: {}", line);
    ingredients.push(line.parse::<i64>().unwrap());
  }
    
  // Check phase
  let mut found_fresh = 0;
  for ingredient in ingredients {
    for range in &fresh_nums {
      if ingredient >= range.0 && ingredient <= range.1 {
        found_fresh += 1;
        break;
      }
    }
  }
  
  println!("Found {} fresh ingredients", found_fresh);
}