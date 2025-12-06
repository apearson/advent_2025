use std::fs;

pub fn start() {
  parse_file();
}

fn parse_file() {
    let input = fs::read_to_string("./src/day6/input").expect("Failed to read input file");
  
    // Lines 
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut ops: Vec<char> = Vec::new();
    
  
    for line in input.lines() {
      if line.contains("*") {
        ops = line.split_ascii_whitespace().map(|s| s.chars().next().unwrap()).collect();
        
        continue;
      }
      
      let line_nums = line.split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        // .map(|s| s.to_string())
        .collect();
      
      numbers.push(line_nums);
    }
    
    let mut final_total = 0;
    
    // Calculate
    for c in 0.. ops.len() {
      let op = ops[c];
      let mut problem_nums: Vec<i64> = Vec::new();
      let mut answer = if op == '*' { 1 } else { 0 };
      
      for r in 0..numbers.len() {
         problem_nums.push(numbers[r][c].clone());
      }
      
      let num_string: Vec<_> = problem_nums.iter().map(|n| n.to_string()).collect();
      
      for num in problem_nums {
        if op == '+' {
          answer += num;
          continue;
        }
        
        if op == '*' {
          answer = answer * num;
          continue;
        }
        
      }
      
      final_total += answer;
      
      println!("Problem {}: [{}] = {}", op, num_string.join(","), answer);
    }
    
    println!("Final Total: {}", final_total);
}