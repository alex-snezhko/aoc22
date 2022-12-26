pub fn part1() -> io::Result<()> {
    let line = read_lines("input6")?.nth(0).unwrap().unwrap();
    let mut num = 0;
    let mut buf = vec![];
    for c in line.chars() {
        num += 1;
        buf.push(c);
        if buf.len() == 4 {
            let mut copy = buf.clone();
            copy.sort();
            copy.dedup();
            if copy.len() == 4 {
                println!("{}", num);
                break;
            } else {
                buf.remove(0);
            }
        }
    }
    Ok(())
}

pub fn part2() -> io::Result<()> {
    let line = read_lines("input6")?.nth(0).unwrap().unwrap();
    let mut num = 0;
    let mut buf = vec![];
    for c in line.chars() {
        num += 1;
        buf.push(c);
        if buf.len() == 14 {
            let mut copy = buf.clone();
            copy.sort();
            copy.dedup();
            if copy.len() == 14 {
                println!("{}", num);
                break;
            } else {
                buf.remove(0);
            }
        }
    }
    Ok(())
}
