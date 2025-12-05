use std::fs;

// Board Rules
const ROLL_CELL: char = '@';
const EMPTY_CELL: char = '.';

pub fn start() {
  let input = fs::read_to_string("./src/day4/input").expect("Failed to read input file");
  
  let mut board = parse_board(input);
  
  let mut total_removed = 0;
  
  // Loop until set of number of rolls stops changing
  loop {
    let (current_accessible_rolls, next_gen) = calculate_accessible(board);
    
    println!("The number of accessible rolls: {}", current_accessible_rolls);
    
    if current_accessible_rolls == 0 {
      break;
    }
    
    board = next_gen;
    total_removed += current_accessible_rolls;
  }
  
  println!("The final number of removed rolls: {}", total_removed);
}

pub fn parse_board(input: String) -> Vec<Vec<char>> {
  let board: Vec<Vec<char>> = 
    input.lines()
         .map(|line| line.trim_end().chars().collect())
         .collect();
  
  return board;
}

pub fn calculate_accessible(board: Vec<Vec<char>>) -> (isize, Vec<Vec<char>>) {
  let mut total_accessible = 0;
  let mut next_gen = board.clone();
  
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
        next_gen[r][c] = '.';
      }
    }
  }
  
  return (total_accessible, next_gen);
}