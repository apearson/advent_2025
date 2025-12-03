use std::fs;

pub fn day2() {
    let input = fs::read_to_string("./src/day2/input").expect("Failed to read input file");
    
    let ranges = input.split(',');
    
    let invalid_ids = ranges.flat_map(find_invalid_ids);
    
    let sum_of_ids: i64 = invalid_ids.sum();
    
    println!("Sum of invalid IDs: {}", sum_of_ids);
}

fn find_invalid_ids(range_string: &str) -> Vec<i64> {
  
  let ranges = range_string.split('-').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
  
  if(ranges.len() != 2) {
    panic!("Invalid range string: {}", range_string);
  }
    
  // Parse out the start and end
  let start = ranges[0];
  let end = ranges[1];
    
  // Get the range as numbers
  let invalid_ids = (start..=end).filter(|&x| is_invalid_id(x)).collect();
  
  return invalid_ids;
}

fn is_invalid_id(id: i64) -> bool {
  // Turn id into string to check for repeating digits
  let id_str = id.to_string();
  let id_str_len = id_str.len();
  
  // 1188511885
  // 10
  // 0..5
  // 
 
  
  // println!("Number is not even digits: {}", id_str);

  // Substring id_str in 2
  let prefix = &id_str[0..((id_str_len/2))];
  let postfix = &id_str[(id_str_len/2)..id_str_len];
  
  let valid = prefix != postfix;
  
  println!("Checking :{} | Prefix: {} | Postfix: {} | Valid: {}", id_str, prefix, postfix, valid);
  
  return valid; // Placeholder logic
}