pub fn part1() -> io::Result<()> {
    let lines = read_lines("input9")?;
    let (mut hx, mut hy) = (0, 0);
    let (mut tx, mut ty) = (0, 0);
    let mut unique_poses = HashSet::new();
    unique_poses.insert((0, 0));
    for line in lines {
        let line = line.unwrap();
        let split = line.splitn(2, " ").collect::<Vec<_>>();
        let (dir, count) = (split[0], split[1]);
        let count = count.parse::<i32>().unwrap();
        for _ in 0..count {
            let (nhx, nhy) = match dir {
                "R" => (hx + 1, hy),
                "L" => (hx - 1, hy),
                "U" => (hx, hy - 1),
                _ => (hx, hy + 1),
            };
            let diff = std::cmp::max(((nhx - tx) as i32).abs(), ((nhy - ty) as i32).abs());
            if diff >= 2 {
                (tx, ty) = (hx, hy);
            }
            (hx, hy) = (nhx, nhy);
            unique_poses.insert((tx, ty));
        }
    }
    println!("{}", unique_poses.len());
    Ok(())
}

pub fn part2() -> io::Result<()> {
    let lines = read_lines("input9")?;
    // let (mut hx, mut hy) = (0, 0);
    let mut segs = (0..10).map(|_| (0, 0)).collect::<Vec<_>>();
    let mut unique_poses = HashSet::new();
    unique_poses.insert((0, 0));
    for line in lines {
        let line = line.unwrap();
        let split = line.splitn(2, " ").collect::<Vec<_>>();
        let (dir, count) = (split[0], split[1]);
        let count = count.parse::<i32>().unwrap();
        for _ in 0..count {
            let (hx, hy) = segs[0];
            let (nhx, nhy) = match dir {
                "R" => (hx + 1, hy),
                "L" => (hx - 1, hy),
                "U" => (hx, hy - 1),
                _ => (hx, hy + 1),
            };

            // let (mut nx, mut ny) = (nhx, nhy);
            // let (mut cx, mut cy) = tails[0];
            let mut new_segs = vec![(nhx, nhy)];
            for si in 1..10 {
                let (cx, cy) = segs[si];
                let (nx, ny) = new_segs[si - 1];
                let dx = ((nx - cx) as i32).abs();
                let dy = ((ny - cy) as i32).abs();
                let diff = std::cmp::max(dx, dy);
                let new_pos = if diff >= 2 {
                    let mx = if cx == nx { 0 } else {
                        if nx > cx { 1 } else { -1 }
                    };
                    let my = if cy == ny { 0 } else {
                        if ny > cy { 1 } else { -1 }
                    };
                    (cx + mx, cy + my)
                } else { segs[si] };
                new_segs.push(new_pos);
            }
            segs = new_segs;
            unique_poses.insert(segs.last().unwrap().to_owned());
        }
    }
    println!("{:?}", segs);
    println!("{}", unique_poses.len());
    Ok(())
}
