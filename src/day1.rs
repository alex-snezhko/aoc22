use crate::*;

pub fn part1() {
    let mut nums = vec![];
    let mut curr_tot = 0;
    let lines = read_lines("./input").unwrap();
    for line in lines {
        if let Ok(line) = line {
            if line == "" {
                nums.push(curr_tot);
                curr_tot = 0;
            } else {
                let num = line.parse::<i32>().unwrap();
                curr_tot += num;
            }
        }
    }
    nums.sort();
    nums.reverse();
    let s = nums[0..3].to_vec();
    println!("{:?}", s);
}

pub fn part2() {
    let mut nums = vec![];
    let mut curr_tot = 0;
    let lines = read_lines("./input").unwrap();
    for line in lines {
        if let Ok(line) = line {
            if line == "" {
                nums.push(curr_tot);
                curr_tot = 0;
            } else {
                let num = line.parse::<i32>().unwrap();
                curr_tot += num;
            }
        }
    }
    nums.sort();
    nums.reverse();
    let s = nums[0..3].to_vec();
    println!("{:?}", s);
}
