pub fn part1() {
    let lines = read_lines("input12").unwrap().map(|x| x.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut pos = (0, 0);
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == 'S' {
                pos = (i as i32, j as i32);
                break;
            }
        }
    }
    let mut q = VecDeque::new();
    let mut result = 0;
    q.push_back((pos, 'a', 0));
    let mut visited = HashSet::new();
    visited.insert(pos);
    'outer: while !q.is_empty() {
        let ((y, x), val, steps) = q.pop_front().unwrap();
        // println!("{} {} {} {}", x, y, val, steps);
        if lines[y as usize][x as usize] == 'E' {
            println!("got here");
            result = steps;
            break 'outer;
        }
        for (dy, dx) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let (ny, nx) = (y + dy, x + dx);
            if nx < 0 || nx as usize >= lines[0].len() || ny < 0 || ny as usize >= lines.len() {
                continue;
            }
            let c = lines[ny as usize][nx as usize];
            let c = if c == 'E' { 'z' } else { c };
            let diff = c as i32 - val as i32;
            if diff <= 1 && !visited.contains(&(ny, nx)) {
                visited.insert((ny, nx));
                q.push_back(((ny, nx), c, steps + 1));
            }
        }
    }
    println!("{}", result);
}

pub fn part2() {
    let lines = read_lines("input12").unwrap().map(|x| x.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    // jank
    let mut starting_poses = vec![];
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == 'S' || lines[i][j] == 'a' {
                starting_poses.push((i as i32, j as i32));
            }
        }
    }
    let mut min = i32::MAX;
    for pos in starting_poses {
        let mut q = VecDeque::new();
        let mut result = 0;
        q.push_back((pos, 'a', 0));
        let mut visited = HashSet::new();
        visited.insert(pos);
        'outer: while !q.is_empty() {
            let ((y, x), val, steps) = q.pop_front().unwrap();
            if lines[y as usize][x as usize] == 'E' {
                result = steps;
                break 'outer;
            }
            for (dy, dx) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let (ny, nx) = (y + dy, x + dx);
                if nx < 0 || nx as usize >= lines[0].len() || ny < 0 || ny as usize >= lines.len() {
                    continue;
                }
                let c = lines[ny as usize][nx as usize];
                let c = if c == 'E' { 'z' } else if c == 'S' { 'a' } else { c };
                let diff = c as i32 - val as i32;
                if diff <= 1 && !visited.contains(&(ny, nx)) {
                    visited.insert((ny, nx));
                    q.push_back(((ny, nx), c, steps + 1));
                }
            }
        }
        if result == 0 {
            continue;
        }
        min = std::cmp::min(min, result);
    }
    println!("{}", min);
}
