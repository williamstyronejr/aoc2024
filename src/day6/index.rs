use std::{collections::HashSet, fs};

#[derive(Eq, Hash, PartialEq,Debug,Clone)]
enum Direction {
  Up,
  Down,
  Left,
  Right
}

fn get_next_move(grid: &Vec<&str>, x: usize, y: usize, dir: &Direction) -> Option<(usize, usize, Direction)> {
  let mut next_x = x;
  let mut next_y = y;
  let mut next_dir = match  dir {
    Direction::Up =>    { Direction::Up}
    Direction::Down =>  { Direction::Down }
    Direction::Left =>  { Direction::Left }
    Direction::Right => { Direction::Right }
  };
  let max_x = grid.first().unwrap().len() - 1;

  if x == 0 && *dir == Direction::Left { return None; }
  if x == max_x && *dir == Direction::Right { return None; }
  if y == 0 && *dir == Direction::Up { return None; }
  if y == grid.len() - 1 && *dir == Direction::Down { return None; }

  match dir {
    Direction::Up => { next_y -= 1; },
    Direction::Down => { next_y += 1; },
    Direction::Left => { next_x -= 1; },
    Direction::Right => { next_x += 1; }, 
  }

  if grid.get(next_y).unwrap().get(next_x..next_x+1).unwrap() == "#" {
    next_dir = match dir {
      Direction::Up => { Direction::Right },
      Direction::Down => { Direction::Left} ,
      Direction::Left => { Direction::Up},
      Direction::Right => { Direction::Down },
    };

    return Some((x,y, next_dir));
  }

  Some((next_x,next_y, next_dir))
}

fn is_loop(grid: &Vec<&str>, start: (usize, usize), new_obs: (usize, usize)) -> bool {
  let mut visited_obs:HashSet<(usize, usize, Direction)> = HashSet::new();
  let mut pos = start;
  let mut dir = Direction::Up;

  loop {
    let attemp = get_next_move(grid, pos.0, pos.1, &dir);
    if attemp.is_none() { break; }

    let mut next_move = attemp.unwrap();

    // if placed obs is hit, change direction
    if next_move.0 == new_obs.0 && next_move.1 == new_obs.1 { 
      next_move = match next_move.2 {
        Direction::Up =>   { (pos.0, pos.1, Direction::Right) },
        Direction::Down => { (pos.0, pos.1, Direction::Left) },
        Direction::Left => { (pos.0, pos.1, Direction::Up) },
        Direction::Right =>{ (pos.0, pos.1, Direction::Down) },
          }
    }

    // Change direction means wall hit, attempt to store
    if next_move.2 != dir {
      let obs_pos = match dir {
        Direction::Up =>    {(next_move.0, next_move.1 - 1, dir)},
        Direction::Down =>  {(next_move.0, next_move.1 + 1, dir)},
        Direction::Left =>  {(next_move.0 - 1, next_move.1, dir)},
        Direction::Right => {(next_move.0 + 1, next_move.1, dir)},
      };

      if !visited_obs.insert(obs_pos) { 
        return true; 
      }
    }

    pos.0 = next_move.0;
    pos.1 = next_move.1;
    dir = next_move.2;
  }
  
  false
}

// Brutal Force
fn total_loop(grid: &Vec<&str>, guard_x: usize, guard_y: usize) -> i32 {
  let mut total_loops = 0;
  let mut x = guard_x;
  let mut y = guard_y;
  let mut dir= Direction::Up;
  let mut obs_pos = HashSet::new();

  loop {
    let next_move = get_next_move(grid, x, y, &dir);
    if next_move.is_none() { break; }

    let next_move = next_move.unwrap();

    let next_obs = (next_move.0, next_move.1);

    if is_loop(grid, (guard_x, guard_y), next_obs) {
      if obs_pos.insert(next_obs) {
        total_loops += 1;
      }
    }

    x = next_move.0;
    y = next_move.1; 
    dir = next_move.2;
  }

  total_loops
}

fn determine_moves(grid: &Vec<&str>, guard_x: usize, guard_y: usize) -> usize {
  let mut moves = HashSet::new();
  let mut pos = ( guard_x, guard_y, Direction::Up);
  
  moves.insert((pos.0, pos.1));

  loop {
    let next_move = get_next_move(grid, pos.0, pos.1, &pos.2);
    if next_move.is_none() { break; }
    
    let next_move = next_move.unwrap();

    moves.insert((next_move.0, next_move.1));
    pos = next_move;
  }

  moves.len()
}

pub fn prob1() {
  let mut grid = Vec::new();
  let mut guard_x = 0;
  let mut guard_y = 0;
  let binding = fs::read_to_string("src/day6/input.txt").unwrap();

  for (idx,line) in binding.lines().enumerate() {
    let guard = line.find("^");
    if guard.is_some() { guard_x = guard.unwrap(); guard_y = idx }
    grid.push(line);
  }

  let part1_ans =  determine_moves(&grid, guard_x, guard_y);
  let part2_ans = total_loop(&grid, guard_x, guard_y);
  

  // 4374
  println!("Part 1: {}", part1_ans);

  // 1705
  println!("Part 2: {}", part2_ans);
}