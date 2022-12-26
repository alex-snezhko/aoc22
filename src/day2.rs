use crate::*;

pub fn part1() {
    let lines = read_lines("./input2").unwrap();
    let mut total = 0;
    for line in lines {
        let line = line.unwrap();
        let res: Vec<&str> = line.split(" ").collect();
        let you = res[0];
        let me = res[1];
        let win = you == "A" && me == "Y" || you == "B" && me == "Z" || you == "C" && me == "X";
        let draw = you == "A" && me == "X" || you == "B" && me == "Y" || you == "C" && me == "Z";
        let choice_points = if me == "X" { 1 } else if me == "Y" { 2 } else { 3 };
        let win_points = if win { 6 } else if draw { 3 } else { 0 };
        println!("{:?} {:?}", line, win);
        let points = choice_points + win_points;
        total += points;
    }
    println!("{:?}", total);
}

pub fn part2() {
    let lines = read_lines("./input2").unwrap();
    let mut total = 0;
    for line in lines {
        let line = line.unwrap();
        let res: Vec<&str> = line.split(" ").collect();
        let you = res[0];
        let me = res[1];
        let win = me == "Z";
        let draw = me == "Y";
        let choice_points = match (you, me) {
            ("A", "X") => 3,
            ("A", "Y") => 1,
            ("A", "Z") => 2,
            ("B", "X") => 1,
            ("B", "Y") => 2,
            ("B", "Z") => 3,
            ("C", "X") => 2,
            ("C", "Y") => 3,
            ("C", "Z") => 1,
            _ => panic!("")
        };
        let win_points = if win { 6 } else if draw { 3 } else { 0 };
        let points = choice_points + win_points;
        total += points;
    }
    println!("{:?}", total);
}
