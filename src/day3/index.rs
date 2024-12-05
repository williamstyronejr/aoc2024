use regex::Regex;
use std::fs;

fn removeDont(line: &str) -> String  {
    // if line.find("don't()").is_none() {
    //     return line.to_string();
    // }

    // let mut last_dont = line.find("don't()").unwrap();
    let mut v: Vec<&str> = Vec::new();
    // v.push(&line[0..last_dont]);
    //
    // loop {
    //     let next_do = match line[last_dont..].find("do()") {
    //         None => {
    //             break;
    //         }
    //         Some(index) => index + last_dont
    //     };
    //     // println!("first dont {} next do{}",last_dont, next_do);
    //
    //     match line[next_do..].find("don't()") {
    //         None => {
    //             v.push(&line[next_do..]);
    //             break;
    //         }
    //         Some(index) => {
    //             // println!("{}, {}", next_do, index);
    //             v.push(&line[next_do..index + next_do]);
    //             last_dont = index + next_do;
    //         }
    //     };
    // }

    let first_dont = line.find("don't()").unwrap();
    let indexs = line.match_indices("do()").map(|(index, _)| index).filter(|&x| x > first_dont).collect::<Vec<_>>();
    // let indexs = line.match_indices("do()").map(|(index, _)| index).collect::<Vec<_>>();

    let mut vec = vec![];
    vec.push(&line[0..first_dont]);

    println!("first dont{}", first_dont);
    // println!("{:?}", indexs);

    // No changes: 24538637
    //

    for i in 0..indexs.len() {
        let right_index = if (i == indexs.len() -1 ) { line.len() } else { indexs[i+1] };
        // println!("index {} right {} max length {}", indexs[i], right_index, indexs.len());
        let next_dont = match line[indexs[i]..right_index].find("don't()") {
            None => {
                right_index
            }
            Some(val) => val + indexs[i]
        } ;

        vec.push(&line[indexs[i]..next_dont]);
    }


    println!("{:?}", vec.join(""));

    // 114385506 To High method 2
    // 89846869 To High
    // 43364998 Too low
    // 76186251 ???
    vec.join("")
}

fn checkValidMul(line: &str) -> Vec<&str> {
    let reg = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut valid_muls = vec![];

    for i in reg.find_iter(line).map(|x| x.as_str()) {
        valid_muls.push(i);
    }

    valid_muls
}

fn getProduct(line: &str) -> i32 {
    let reg = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut mul_values = 0;

    for item in reg.find_iter(line).map(|x| x.as_str()) {

        let left_idx = item.find("(").unwrap() + 1;
        let nums = &item[left_idx..item.len() - 1]
            .split(",")
            .collect::<Vec<&str>>();

        mul_values += nums[0].parse::<i32>().unwrap() * nums[1].parse::<i32>().unwrap();
    }

    mul_values
}

pub fn prob1() {
    let mut part1_ans = 0;
    let mut part2_ans = 0;

    for line in fs::read_to_string("src/day3/input.txt").unwrap().lines() {
        // part1_ans += getProduct(line);
        part2_ans += getProduct(removeDont(line).as_str());
        println!("{}", part2_ans);
        // valid_muls_2.append(&mut checkValidMul(removeDont(line).as_str()));
    }

    // println!("Valid muls 2: {:?}", valid_muls_2);
    // 178794710
    println!("part1: {}", part1_ans);

    //
    println!("part2: {}", part2_ans);
    // let mut part1_ans = 0;
    // let mut part2_ans = 0;
    //
    // for item in valid_muls {
    //     let left_idx = item.find("(").unwrap() + 1;
    //     let nums = &item[left_idx..item.len() - 1]
    //         .split(",")
    //         .collect::<Vec<&str>>();
    //
    //     println!("{:?}", nums);
    //     part1_ans += nums[0].parse::<i32>().unwrap() * nums[1].parse::<i32>().unwrap();
    // }
    //
    // // 178794710
    // println!("Part 1: {}", part1_ans);
    //
    // //
    // println!("Part 2: {}", part2_ans);
    // // println!("Valid muls: {:?}", valid_muls);
}
