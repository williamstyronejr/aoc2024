use std::fs;

fn isValidReport(report: &Vec<i32>) -> bool {
    let level_diff = report[0] - report[1];

    // Neither increase nor decrease is invalid
    if level_diff == 0 { return false; }
    let direction = if level_diff > 0 { 1 } else { -1 };

    for i in 0..report.len() - 1 {
        let diff = report[i] - report[i + 1];

        if diff.abs() < 1 || diff.abs() > 3 { return false; }

        if diff > 0 && direction == -1  {
            return false;
        } else if diff < 0 && direction == 1 {
            return false;
        }
    }

    true
}

pub fn prob1() {
    let mut reports  = Vec::new();

    for line in fs::read_to_string("src/day2/input.txt").unwrap().lines() {
        let mut vec = line.split_whitespace().collect::<Vec<&str>>();

        reports.push(vec.iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    }

    let mut part1_ans = 0;
    let mut part2_ans = 0;

    for report in reports {
        if isValidReport(&report) {
            part1_ans = part1_ans + 1;
            part2_ans = part2_ans + 1;
        } else {
            // Part 2 (only need to check unsafe reports for removal)
            for i in 0..report.len() {
                let mut clone = report.clone();
                clone.remove(i);

                if isValidReport(&clone) {
                    part2_ans = part2_ans + 1;
                    break;
                }
            }

        }
    }

    // 663
    println!("Safe Reports Without Removal = {:?}", part1_ans);

    // 692
    println!("Safe Reports With Removal = {:?}", part2_ans);
}
