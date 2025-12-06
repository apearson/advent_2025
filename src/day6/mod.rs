use std::fs;
use itertools::Itertools;

pub fn start() {
    let input = fs::read_to_string("./src/day6/input").expect("Failed to read input file");
  
    // Lines 
    let mut columns: Vec<Vec<char>> = Vec::new(); // 
    let mut ops: Vec<char> = Vec::new();
    
    let lines:Vec<_>= input.lines().collect();
    let num_of_lines = input.lines().count();
    
    // Parse
    for r in 0..num_of_lines {
      let line = lines[r];
      
      if line.contains("*") {
        ops = line.chars().collect();
        
        continue;
      }
      
      let line_nums = line.chars().collect::<Vec<char>>();
      
      for c  in 0..line_nums.len() {
        if columns.len() <= c {
          columns.push(Vec::new());
        }
        
        if columns[c].len() <= r {
          columns[c].push(' ');
        } 
        
        columns[c][r] = line_nums[c];
      }
    }
    
    let mut final_total = 0;
    
    let mut current_numbers: Vec<i64> = Vec::new();
    
    // Calculate
    for c in (0.. ops.len()).rev() {
      let op = ops[c];
      let column_nums = columns[c].clone();
      let num_string = column_nums.into_iter().join("");
      
      let trimmed_num_string = num_string.trim();
      
      if !trimmed_num_string.is_empty() {
        let parsed_num = trimmed_num_string.parse::<i64>().unwrap();
        
        current_numbers.push(parsed_num);
      }
      
      // has op code
      if !op.is_whitespace() {
        println!("End of problem, op: {} numbers {} to final {}", op, current_numbers.clone().into_iter().join(","), final_total);
        
        
        let mut answer = 0;
        if op == '+' {
          for num in current_numbers.clone() {
            answer += num;
          }
          
          final_total += answer;
        }
        
        if op == '*' {
          answer = 1; // Start at 1 (0 * anything = 0)
          for num in current_numbers.clone() {
            answer = answer * num;
          }
          
          final_total += answer;
        }
        
        current_numbers.clear();
        continue;
      }
    }
    
    println!("Final Total: {}", final_total);
}