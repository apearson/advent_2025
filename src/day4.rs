use std::fs;

// Board Rules
const ROLL_CELL: char = '@';
const EMPTY_CELL: char = '.';

pub fn start() {
  let input = fs::read_to_string("./src/day4/input").expect("Failed to read input file");
  
  let board = parse_board(input);
  
  let num_of_accessible_rolls = calculate_accessible(board);
  
  println!("The number of accessible rolls: {}", num_of_accessible_rolls);
  
}

pub fn parse_board(input: String) -> Vec<Vec<char>> {
  let board: Vec<Vec<char>> = 
    input.lines()
         .map(|line| line.trim_end().chars().collect())
         .collect();
  
  return board;
}

pub fn calculate_accessible(board: Vec<Vec<char>>) -> isize {
  let mut total_accessible = 0;
  let height = board.len() as isize;
  
  // Directions 
  let directions: [(isize, isize); 8]  = [
    (-1, -1), (-1, 0), (-1, 1),
    (0,  -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
  ];

  // Loop over positions
  for (r, row) in board.iter().enumerate(){
    let width = row.len() as isize;
    
    for (c, &cell) in row.iter().enumerate() {
      // Check for roll in cell
      if cell == EMPTY_CELL {
        continue;
      }
      
      // Count of neighbors with rolls
      let mut neighbors: isize = 0;
      
      // Check all directions
      for (r_y, r_x) in directions {
        let r = r as isize + r_y;
        let c = c as isize + r_x;
        
        // Check out of bounds
        if r >= 0 && r < height && c >=0 && c < width {
          
          // Check for roll
          if board[r as usize][c as usize] == ROLL_CELL {
            neighbors += 1;
          }
        }
      }
      
      if neighbors < 4 {
        total_accessible += 1;
      }
    }
  }
  
  return total_accessible;
}