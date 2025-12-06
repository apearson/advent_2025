use std::fs;

pub fn start() {
  let input = fs::read_to_string("./src/day5/input").expect("Failed to read input file");

  // Parse Phase
  let mut fresh_nums= parse_lines(input);
  let fresh_nums_len = fresh_nums.len();
  
  // Sort ranges by starting
  fresh_nums.sort_by(|a, b| a.0.cmp(&b.0));

  // List with only non overlapping ranges
  let mut new_list: Vec<(i64, i64)> = Vec::new();
    
  // Start with the first
  new_list.push(fresh_nums[0]);
  
  // Loop over range 
  for i in 1..(fresh_nums_len) {
    let current_range = new_list[new_list.len() - 1];
    let item = fresh_nums[i];
    
    println!("Working on {}-{} & {}-{}", current_range.0, current_range.1, item.0, item.1);
    
    // (push) Ranges are non overlapping (done)
    if item.0 > current_range.1 {
      new_list.push(item);
      
      continue;
    }
    
    // (pop and extend) Ranges are partial overlapping
    if item.0 <= current_range.1 && item.1 > current_range.1 {
      new_list.pop();
      
      new_list.push((current_range.0, item.1));
      
      continue;
    }
    
    // (do nothing) item is fully inside current_range (done)
    if item.0 >= current_range.0 && item.1 <= current_range.1 {
      continue;
    }
    
    println!("Case not found for {}-{} & {}-{}", current_range.0, current_range.1, item.0, item.1);
  }
  
  // New ranges
  // let mut removed = true;
  
  // // Loop while we're still combining rangings
  // while removed { 
  //   let fresh_nums_len = fresh_nums.len();

  //   removed = false;
    
  //   // Part 2 Reduce the overlapping ranges 
  //   'target_loop: for i in 0..fresh_nums_len {
  //     for j in 0..fresh_nums_len {
  //       // Skipping ourselves
  //       if j == i {
  //         continue;
  //       }
        
  //       // Finding a replacement range
  //       let replacement_range = reduce_ranges(fresh_nums[i], fresh_nums[j]);
        
  //       // 0: Non overlapping
  //       // 1: Remove only target range, complete inside compare range
  //       // 2: Remove both ranges, new range replaces both
  //       match replacement_range.0 {
  //         RemoveType::None => {
  //           continue;
  //         }
  //         RemoveType::Target => {
  //           println!("Removing target");
            
  //           fresh_nums.remove(i);
  //           removed = true;
            
  //           break 'target_loop;
  //         }
  //         RemoveType::Both => {
  //           println!("Removing both, replacing with: {}-{}", replacement_range.1.unwrap().0, replacement_range.1.unwrap().1);
  //           fresh_nums.remove( cmp::max(i, j));
  //           fresh_nums.remove(cmp::min(i,j));
  //           fresh_nums.push(replacement_range.1.unwrap());
  //           removed = true;
            
  //           break 'target_loop;
  //         },
  //       }
  //     }
  //   }
  // }
  
  // Printing Completed List
  for range in &new_list {
    println!("{}-{}", range.0, range.1);
  }
  
  // Part 2 Count Phase 
  println!("Total Fresh Items: {}", count_total(new_list));
    
  // Check phase
  // let mut found_fresh = 0;
  // for ingredient in ingredients {
  //   for range in &fresh_nums {
  //     if ingredient >= range.0 && ingredient <= range.1 {
  //       found_fresh += 1;
  //       break;
  //     }
  //   }
  // }
  
  // println!("Found {} fresh ingredients", found_fresh);
}

fn parse_lines(input: String) -> Vec<(i64, i64)> {
  let mut fresh_nums: Vec<(i64, i64)> = Vec::new();

  for line in input.lines() {
    
    if line.trim().is_empty() {
      continue;
    }
    
    // Phase 1
    if line.contains('-') {
      let ranges: Vec<i64> = line.split('-').map(|n| n.parse().unwrap()).collect();
      
      // println!("Found Range: {} to {}", ranges[0], ranges[1]);
      fresh_nums.push((ranges[0], ranges[1]));
      continue;
    }
  }
  
  return fresh_nums;
}

fn count_total(ranges: Vec<(i64, i64)> ) -> i64 {
  let mut total_fresh = 0;
  
  for range in ranges {    
    total_fresh += range.1 - range.0 + 1;
  }
  
  return total_fresh;
}

// fn reduce_ranges(target: (i64, i64), compare_range: (i64, i64)) -> (RemoveType, Option<(i64, i64)>) {
//   let target_min = target.0;
//   let target_max = target.1;
//   let compare_min = compare_range.0;
//   let compare_max = compare_range.1;
 
//   // Check target completely inside compare
//   if target_min >= compare_min && target_max <= compare_max {
//       println!("{},{} completely inside {},{}", target_min, target_max, compare_min, compare_max);
//       return (RemoveType::Target, None);
//   }
  
//   // Check compare complete inside target
//   // if compare_min >= target_min && compare_max <= target_max {
//   //     println!("Range {},{} completely inside {},{}", compare_min, compare_max, target_min, target_max);
//   //     return Some(target);
//   // }
  
//   // Compare max overlaps with Target min on lower ([compare_min, target_min, compare_max, target_max])
//   // if target_min > compare_min && target_max >= compare_max {
//   //   println!("Range low: {},{} overlapping high: {},{}", compare_min, compare_max, target_min, target_max);
    
//   //   return Some((compare_min, target_max));
//   // }
    
//   // Target max overlaps with Compare min on lower ([target_min, compare_min, target_max, compare_max])
//   if compare_min >= target_min && compare_min <= target_max && compare_max >= target_min {
//     println!("{},{} overlapping compare: {},{}", target_min, target_max, compare_min, compare_max);
        
//     return (RemoveType::Both, Some((target_min, compare_max)));
//   }
  
//   // No overlap returning orginal target range
//   println!("{},{} non-overlapping compare: {},{}", target_min, target_max, compare_min, compare_max);
//   return (RemoveType::None, None);
  
// }