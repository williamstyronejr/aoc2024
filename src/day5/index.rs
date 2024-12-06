use std::{collections::HashMap, fs};

fn get_invalid_index(pairs: &HashMap<String, String>, update:&str) -> Option<usize> {
  for (idx, page) in update.split(',').enumerate() {
    if idx == 0 { continue; }
    
    let left_size = update[0..update.find(page).unwrap()-1].split(",").collect::<Vec<_>>();

    for item in left_size {
      match pairs.get(page) {
        None => {},
        Some(checks) => {
            if checks.contains(item) { 
              return Some(idx);
            }
        }
      }
    }
  }

  None
}

fn get_reformed_updates(pairs: &HashMap<String, String>, updates: Vec<& str>) -> Vec<String> {
  let mut reformed_updates = Vec::new();

  for update in updates {
    if get_invalid_index(pairs, update).is_none() { continue; }
    
    let mut pages = update.split(",").collect::<Vec<_>>();

    loop {
      let idx =  get_invalid_index(pairs, &pages.join(","));
      
      if idx.is_none() { break; }

      pages.swap(idx.unwrap(), idx.unwrap() - 1);
    }

    reformed_updates.push(pages.join(","));
  }

  reformed_updates
}

fn get_valid_updates<'a>(pairs: &HashMap<String, String>, updates: Vec<&'a str>) -> Vec<&'a str> { 
  let mut valid_updates = Vec::new();

  for update in updates {
    if get_invalid_index(pairs, update).is_none() {
      valid_updates.push(update)
    }
  }

  valid_updates
}

pub fn prob1() {
  let mut order_pairs = HashMap::new();
  let mut updates = Vec::new();
  let mut part1_ans = 0;
  let mut part2_ans = 0;
  
  let binding = fs::read_to_string("src/day5/input.txt").unwrap();

  for line in binding.lines() {
      if line.contains("|") {
        let pair = line.split('|').collect::<Vec<_>>();
        order_pairs.entry(pair[0].to_string()).and_modify(|x: &mut String| x.push_str(&format!(" {}", pair[1]))).or_insert(pair[1].to_string());
        // order_pairs.push(line);
      } else if line.contains(",") {
        updates.push(line);
      }
  }

  let valid_updates = get_valid_updates(&order_pairs, updates.clone());
  let reformed_updates = get_reformed_updates(&order_pairs, updates.clone());

  for update in valid_updates {
    let pages = update.split(",").collect::<Vec<_>>();
    part1_ans += pages[(pages.len() - 1)/2].parse::<i32>().unwrap();
  }

  for update in reformed_updates {
    let pages = update.split(",").collect::<Vec<_>>();
    part2_ans += pages[(pages.len() - 1)/2].parse::<i32>().unwrap();
  }

  // 5732
  println!("Part 1: {}", part1_ans);

  // 4716
  println!("Part 2: {}", part2_ans);
}