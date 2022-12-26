pub fn part1() -> io::Result<()> {
    let lines = read_lines("input5")?;
    let mut crates_lines = vec![];
    let mut crates_steps = vec![];
    let mut done_crates = false;
    for line in lines {
        let line = line.unwrap();
        if line == "" {
            crates_lines.pop();
            done_crates = true;
        } else if !done_crates {
            crates_lines.push(line);
        } else {
            crates_steps.push(line);
        }
    }
    let mut stacks = (0..9).map(|_| vec![]).collect::<Vec<Vec<char>>>();
    for line in crates_lines {
        let mut rem_line = line.as_str();
        for i in 0..9 {
            let (add, rest) = rem_line.split_at(std::cmp::min(rem_line.len(), 4));
            let c = add.chars().nth(1).unwrap();
            if c != ' ' {
                stacks[i].insert(0, c);
            }
            rem_line = rest;
        }
    }

    for line in crates_steps {
        let line = line.strip_prefix("move ").unwrap();
        let split = line.split(" from ").collect::<Vec<&str>>();
        let (first, second) = (split[0], split[1]);
        let split = second.split(" to ").collect::<Vec<&str>>();
        let (second, third) = (split[0], split[1]);
        let (num_move, move_from, move_to) = (first.parse::<usize>().unwrap(), second.parse::<usize>().unwrap(), third.parse::<usize>().unwrap());

        for _ in 0..num_move {
            let val = stacks[move_from - 1].pop().unwrap();
            stacks[move_to - 1].push(val);
        }
    }
    let result: String = stacks.iter().map(|v| v.last().unwrap()).collect();
    println!("{:?}", result);
    Ok(())
}

pub fn part2() -> io::Result<()> {
    let lines = read_lines("input5")?;
    let mut crates_lines = vec![];
    let mut crates_steps = vec![];
    let mut done_crates = false;
    for line in lines {
        let line = line.unwrap();
        if line == "" {
            crates_lines.pop();
            done_crates = true;
        } else if !done_crates {
            crates_lines.push(line);
        } else {
            crates_steps.push(line);
        }
    }
    let mut stacks = (0..9).map(|_| vec![]).collect::<Vec<Vec<char>>>();
    for line in crates_lines {
        let mut rem_line = line.as_str();
        for i in 0..9 {
            let (add, rest) = rem_line.split_at(std::cmp::min(rem_line.len(), 4));
            let c = add.chars().nth(1).unwrap();
            if c != ' ' {
                stacks[i].insert(0, c);
            }
            rem_line = rest;
        }
    }

    for line in crates_steps {
        let line = line.strip_prefix("move ").unwrap();
        let split = line.split(" from ").collect::<Vec<&str>>();
        let (first, second) = (split[0], split[1]);
        let split = second.split(" to ").collect::<Vec<&str>>();
        let (second, third) = (split[0], split[1]);
        let (num_move, move_from, move_to) = (first.parse::<usize>().unwrap(), second.parse::<usize>().unwrap(), third.parse::<usize>().unwrap());

        let (left, to_add) = stacks[move_from - 1].split_at(stacks[move_from - 1].len() - num_move);
        let left = left.to_vec();
        let mut add = to_add.to_vec().clone();
        stacks[move_to - 1].append(&mut add);
        stacks[move_from - 1] = left;
    }
    let result: String = stacks.iter().map(|v| v.last().unwrap()).collect();
    println!("{:?}", result);
    Ok(())
}