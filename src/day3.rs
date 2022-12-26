pub fn part1() -> io::Result<()> {
    let lines = read_lines("input3")?;
    let mut tot = 0;
    for line in lines {
        let line = line.unwrap();
        let len = line.len();
        let half = len / 2;
        let first = &line[..half];
        let second = &line[half..];
        let mut score = 0;
        for c in first.chars() {
            if second.chars().collect::<Vec<char>>().contains(&c) {
                score += if c.is_uppercase() { c as i32 - 64 + 26 } else { c as i32 - 96 };
                break;
            }
        }
        tot += score;
    }
    println!("{:?}", tot);
    Ok(())
}

pub fn part2() -> io::Result<()> {
    let lines = read_lines("input3")?;
    let mut tot = 0;
    let mut vec = vec![];
    let mut iter = lines.into_iter().peekable();
    while iter.peek().is_some() {
        vec.push((iter.next().unwrap().unwrap(), iter.next().unwrap().unwrap(), iter.next().unwrap().unwrap()))
    }
    for (a, b, c) in vec {
        let mut score = 0;
        for ch in a.chars() {
            if b.chars().collect::<Vec<char>>().contains(&ch) && c.chars().collect::<Vec<char>>().contains(&ch) {
                score += if ch.is_uppercase() { ch as i32 - 64 + 26 } else { ch as i32 - 96 };
                break;
            }
        }
        tot += score;

    }
    println!("{:?}", tot);
    Ok(())
}

