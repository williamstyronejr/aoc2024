use std::fs;

fn getTotalDistance(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut total_distance = 0;
    for i in 0..left.len() {
        let val = left[i] - right[i];
       total_distance += val.abs();
    }

    total_distance
}

fn getSimilarityScore(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut similarity_score = 0;

    for x in left {
        let count = right.iter().filter(|&y| x == y).count();
        similarity_score += x * count as i32;
    }

    similarity_score
}

pub fn prob1() {
    println!("prob1");
    let mut vec_left = vec![];
    let mut vec_right = vec![];

    for line in fs::read_to_string("src/day1/input.txt").unwrap().lines() {
        let v: Vec<&str> = line.split_whitespace().collect();

        vec_left.push(v[0].parse::<i32>().unwrap());
        vec_right.push(v[1].parse::<i32>().unwrap());
    }

    vec_left.sort();
    vec_right.sort();

    let part1_ans = getTotalDistance(&vec_left, &vec_right);
    let part2_ans = getSimilarityScore(&vec_left, &vec_right);

    // 1110981
    println!("Total Distance Score = {:?}", part1_ans);

    // 24869388
    println!("Total Similarity Score = {:?}", part2_ans);
}
