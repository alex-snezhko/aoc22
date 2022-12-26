use std::cmp::{min, max};

fn part1() {
    let lines = read_lines("input18").unwrap();
    let coords = lines.map(|line| {
        let l = line.unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        (l[0], l[1], l[2])
    }).collect::<HashSet<_>>();

    let mut tot = 0;
    for (x, y, z) in &coords {
        let deltas = [(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)];
        let sides_exposed = 6 - deltas
            .iter()
            .filter(|(dx, dy, dz)| coords.contains(&(x + dx, y + dy, z + dz)))
            .count();
        tot += sides_exposed;
    }
    println!("{}", tot);
}

fn part2() {
    let lines = read_lines("input18").unwrap();
    let coords = lines.map(|line| {
        let l = line.unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        (l[0], l[1], l[2])
    }).collect::<HashSet<_>>();

    let (min_x, max_x, min_y, max_y, min_z, max_z) = coords
        .iter()
        .fold(
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN, i32::MAX, i32::MIN),
            |(min_x, max_x, min_y, max_y, min_z, max_z), (x, y, z)| (
                min(min_x, *x),
                max(max_x, *x),
                min(min_y, *y),
                max(max_y, *y),
                min(min_z, *z),
                max(max_z, *z)
            )
        );

    let mut bubble_spaces = HashSet::new();

    let mut tot = 0;
    for (x, y, z) in &coords {
        let deltas = [(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)];
        for (dx, dy, dz) in &deltas {
            let coord = (x + dx, y + dy, z + dz);
            if !coords.contains(&coord) && !bubble_spaces.contains(&coord) {
                let mut visited = HashSet::new();
                visited.insert(coord);
                let mut q = VecDeque::new();
                q.push_back(coord);
                let mut is_bubble = true;
                'outer: while !q.is_empty() {
                    let (cx, cy, cz) = q.pop_front().unwrap();
                    for (dx, dy, dz) in &deltas {
                        let new_coord = (cx + dx, cy + dy, cz + dz);
                        let (nx, ny, nz) = new_coord;
                        if !visited.contains(&new_coord) && !coords.contains(&new_coord) {
                            visited.insert(new_coord);
                            q.push_back(new_coord);
                            if nx >= max_x || nx <= min_x || ny >= max_y || ny <= min_y || nz >= max_z || nz <= min_z {
                                is_bubble = false;
                                break 'outer;
                            }
                        }
                    }
                }
                if is_bubble {
                    bubble_spaces.extend(visited);
                }
            }
        }
        let sides_exposed = 6 - deltas
            .iter()
            .filter(|(dx, dy, dz)| {
                let coord = (x + dx, y + dy, z + dz);
                coords.contains(&coord) || bubble_spaces.contains(&coord)
            })
            .count();
        tot += sides_exposed;
    }
    println!("{}", tot);
}
