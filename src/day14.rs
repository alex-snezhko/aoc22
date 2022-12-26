pub fn part1() {
    let lines = read_lines("input14").unwrap();
    let mut occupied = HashSet::new();
    let (mut min_x, mut max_x, mut max_y) = (i32::MAX, i32::MIN, i32::MIN);
    for line in lines {
        let line = line.unwrap();
        let parts = line
            .split(" -> ")
            .map(|l| {
                let mut ps = l.split(",").map(|p| p.parse::<i32>().unwrap());
                (ps.next().unwrap(), ps.next().unwrap())
            })
            .collect::<Vec<_>>();
        parts.iter().for_each(|(x, y)| {
            min_x = std::cmp::min(min_x, *x);
            max_x = std::cmp::max(max_x, *x);
            max_y = std::cmp::max(max_y, *y);
        });

        let mut iter = parts.into_iter();
        let mut from = iter.next().unwrap();
        while let Some(to) = iter.next() {
            let (sx, sy) = from;
            let (ex, ey) = to;
            let (dx, dy) = if sx == ex { if sy < ey { (0, 1) } else { (0, -1) } } else { if sx < ex { (1, 0) } else { (-1, 0) } };
            let (mut x, mut y) = from;
            while x != ex || y != ey {
                occupied.insert((x, y));
                x += dx;
                y += dy;
            }
            occupied.insert((x, y));
            from = to;
        }
    }
    let mut tot = 0;
    'outer: loop {
        let (mut x, mut y) = (500, 0);
        loop {
            let trials = [(0, 1), (-1, 1), (1, 1)];
            match trials.iter().find(|(dx, dy)| !occupied.contains(&(x + dx, y + dy))) {
                Some((dx, dy)) => {
                    let (new_x, new_y) = (x + dx, y + dy);
                    if new_x < min_x || new_x > max_x || new_y > max_y {
                        println!("{} {} {} {} {}", new_x, new_y, min_x, max_x, max_y);
                        break 'outer;
                    }
                    x = new_x;
                    y = new_y;
                },
                None => {
                    tot += 1;
                    occupied.insert((x, y));
                    break;
                },
            };
        }
    }
    println!("{}", tot);
}

pub fn part2() {
    let lines = read_lines("input14").unwrap();
    let mut occupied = HashSet::new();
    let mut max_y = i32::MIN;
    for line in lines {
        let line = line.unwrap();
        let parts = line
            .split(" -> ")
            .map(|l| {
                let mut ps = l.split(",").map(|p| p.parse::<i32>().unwrap());
                (ps.next().unwrap(), ps.next().unwrap())
            })
            .collect::<Vec<_>>();
        parts.iter().for_each(|(_, y)| {
            max_y = std::cmp::max(max_y, *y + 2);
        });

        let mut iter = parts.into_iter();
        let mut from = iter.next().unwrap();
        while let Some(to) = iter.next() {
            let (sx, sy) = from;
            let (ex, ey) = to;
            let (dx, dy) = if sx == ex { if sy < ey { (0, 1) } else { (0, -1) } } else { if sx < ex { (1, 0) } else { (-1, 0) } };
            let (mut x, mut y) = from;
            while x != ex || y != ey {
                occupied.insert((x, y));
                x += dx;
                y += dy;
            }
            occupied.insert((x, y));
            from = to;
        }
    }
    let mut tot = 0;
    'outer: loop {
        let (mut x, mut y) = (500, 0);
        loop {
            let trials = [(0, 1), (-1, 1), (1, 1)];
            let stopped = match trials.iter().find(|(dx, dy)| !occupied.contains(&(x + dx, y + dy))) {
                Some((dx, dy)) => {
                    let (new_x, new_y) = (x + dx, y + dy);
                    x = new_x;
                    y = new_y;
                    new_y >= max_y - 1
                },
                None => true,
            };

            if stopped {
                tot += 1;
                if x == 500 && y == 0 {
                    break 'outer;
                }
                occupied.insert((x, y));
                break;
            }
        }
    }
    println!("{}", tot);
}
