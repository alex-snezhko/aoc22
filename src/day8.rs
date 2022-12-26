fn next_ij(i: i32, j: i32, (di, dj): (i32, i32)) -> (i32, i32) {
    (i + di, j + dj)
}

pub fn part1() -> io::Result<()> {
    let lines = read_lines("input8")?;
    let lines = lines.collect::<Vec<_>>().into_iter().map(|x| x.unwrap().chars().map(|c| c.to_digit(10)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let len = lines[0].len();
    let mut tot = (len - 1) * 4;
    for i in 1..len-1 {
        for j in 1..len-1 {
            let c = lines[i][j];
            let mut visible = false;
            for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let mut this_way_visible = true;
                let (mut ci, mut cj) = next_ij(i as i32, j as i32, delta);
                while ci >= 0 && ci < len as i32 && cj >= 0 && cj < len as i32 {
                    if lines[ci as usize][cj as usize] >= c {
                        this_way_visible = false;
                        break;
                    }
                    (ci, cj) = next_ij(ci, cj, delta);
                }
                if this_way_visible {
                    visible = true;
                    break;
                }
            }
            if visible {
                tot += 1;
            }
        }
    }
    println!("{}", tot);
    Ok(())
}

pub fn part2() -> io::Result<()> {
    let lines = read_lines("input8")?;
    let lines = lines.collect::<Vec<_>>().into_iter().map(|x| x.unwrap().chars().map(|c| c.to_digit(10)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let len = lines[0].len();
    let mut max = 0;
    for i in 1..len-1 {
        for j in 1..len-1 {
            let c = lines[i][j];
            let mut this_scores = vec![];
            for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let mut dist = 0;
                let (mut ci, mut cj) = next_ij(i as i32, j as i32, delta);
                while ci >= 0 && ci < len as i32 && cj >= 0 && cj < len as i32 {
                    if lines[ci as usize][cj as usize] >= c {
                        dist += 1;
                        break;
                    }
                    (ci, cj) = next_ij(ci, cj, delta);
                    dist += 1;
                }
                this_scores.push(dist);
            }
            let score = this_scores.into_iter().product();
            max = std::cmp::max(max, score);
        }
    }
    println!("{}", max);
    Ok(())
}
