use std::fs;

fn count_word(lines: &Vec<&str>, word: &str) -> usize {
    let mut num_of_occurences = 0;
    let word_rev = word.chars().rev().collect::<String>();
    let first= word.chars().next().unwrap();
    let last = word.chars().rev().next().unwrap();

    for (line_idx, line) in lines.iter().enumerate() {
        // Searchs left/right
        let left_occ = line.match_indices(first).map(|(x,_)| x).collect::<Vec<usize>>();
        let rev_occ = line.match_indices(last).map(|(x,_)| x).collect::<Vec<usize>>();

        // println!("left {:?} right {:?} occs {}", left_occ, rev_occ, num_of_occurences);
       
        for idx in left_occ {
            // Down Search
            if line_idx + word.len() <= lines.len() {
                let mut down_word =Vec::new();
                for i in 0..word.len() {
                    down_word.push(lines[line_idx + i].chars().nth(idx).unwrap());
                }
                if down_word.iter().collect::<String>() == word { num_of_occurences += 1 }
            } 
            if idx + word.len() > line.len() { continue; }

            // Left To Right
            if line[idx..word.len() + idx] == *word {
                num_of_occurences += 1;
            }  

            // Diagonal left to down
            if line_idx + word.len() <= lines.len() {
                let mut temp_word = Vec::new();

                for i in 0..word.len() {
                    temp_word.push(lines[line_idx + i].chars().nth(idx + i).unwrap());
                }

                if temp_word.iter().collect::<String>() == word { num_of_occurences += 1 }
            }

            // Diagonal left up
            if line_idx >= word.len() - 1  {
                let mut temp_word = Vec::new();

                for i in 0..word.len() {
                    temp_word.push(lines[line_idx - i].chars().nth(idx + i).unwrap());
                }

                if temp_word.iter().collect::<String>() == word { num_of_occurences += 1 }
            }

        }

        for idx in rev_occ {
            if line_idx + word_rev.len() <= lines.len() {
                let mut down_word = Vec::new();
                for i in 0..word_rev.len() {
                    down_word.push(lines[line_idx + i].chars().nth(idx).unwrap());
                }
                if down_word.iter().collect::<String>() == word_rev { num_of_occurences += 1 }
            } 

            if idx + word.len() > line.len() { continue; }

            if line[idx..word.len() + idx] == word_rev {
                num_of_occurences += 1;
            } 


            // Diagonal left to down
            if line_idx + word.len() <= lines.len() {
                let mut temp_word = Vec::new();

                for i in 0..word.len() {
                    temp_word.push(lines[line_idx + i].chars().nth(idx + i).unwrap());
                }

                if temp_word.iter().collect::<String>() == word_rev { num_of_occurences += 1 }
            }

            // Diagonal left up
            if line_idx >= word.len() - 1  {
                let mut temp_word = Vec::new();

                for i in 0..word.len() {
                    temp_word.push(lines[line_idx - i].chars().nth(idx + i).unwrap());
                }

                if temp_word.iter().collect::<String>() == word_rev { num_of_occurences += 1 }
            }
        }

    }

    num_of_occurences
}

fn find_cross (lines: Vec<&str>)  -> usize{ 
    let mut num_of_occurences = 0;

    for (line_index, line) in lines.iter().enumerate() {
        if  line_index < 1 || line_index + 1 >= lines.len() { continue; }

        let a_matches = line.match_indices("A").map(|(x,_)| x).collect::<Vec<usize>>();

        for a_idx in a_matches {
            if a_idx < 1 || a_idx == line.len() - 1 { continue; }

            // Get corner chars
            let left_top = lines[line_index - 1].chars().nth(a_idx - 1).unwrap();
            let right_top = lines[line_index - 1].chars().nth(a_idx + 1).unwrap();
            let left_bottom = lines[line_index + 1].chars().nth(a_idx - 1).unwrap();
            let right_bottom = lines[line_index + 1].chars().nth(a_idx + 1).unwrap();

            if (left_top == 'S' && right_bottom == 'M') || (left_top == 'M' && right_bottom == 'S') {
                if (right_top == 'S' && left_bottom == 'M') || (right_top == 'M' && left_bottom == 'S') {
                    num_of_occurences += 1
                }
            }       
        }
    }

    num_of_occurences
}

pub fn prob1() {
    let mut vec: Vec<&str> = Vec::new();

    let binding = fs::read_to_string("src/day4/input.txt").unwrap();
    for line in binding.lines() {
        vec.push(line);
    }

    let part1_ans = count_word(&vec, "XMAS");
    let part2_ans = find_cross(vec);

    // 2718
    println!("Part 1: {:?}", part1_ans);

    // 2046
    println!("Part 2: {:?}", part2_ans);
}