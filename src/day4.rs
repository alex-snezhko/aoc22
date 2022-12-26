fn parts(duty: &str) -> (i32, i32) {
    let parts = duty.split("-").collect::<Vec<_>>();
    (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap())
}

pub fn part1() -> io::Result<()> {
    let lines = read_lines("input4")?;
    let mut tot = 0;
    for line in lines {
        let line = line.unwrap();
        let split = line.split(",").collect::<Vec<_>>();
        let first = split[0];
        let second = split[1];
        let (fb, fe) = parts(first);
        let (sb, se) = parts(second);
        if fb <= sb && fe >= se || sb <= fb && se >= fe {
            tot += 1;
        }
    }
    println!("{:?}", tot);
    Ok(())
}

pub fn part2() -> io::Result<()> {
    let lines = read_lines("input4")?;
    let mut tot = 0;
    for line in lines {
        let line = line.unwrap();
        let split = line.split(",").collect::<Vec<_>>();
        let first = split[0];
        let second = split[1];
        let (fb, fe) = parts(first);
        let (sb, se) = parts(second);
        if fb <= sb && fe >= sb || sb <= fb && se >= fb {
            tot += 1;
        }
    }
    println!("{:?}", tot);
    Ok(())
}

