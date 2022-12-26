pub fn part1() {
    let lines = read_lines("input10").unwrap();
    let report_cycles = [20, 60, 100, 140, 180, 220];
    let mut report_i = 0;
    let mut cycle = 1;
    let mut x = 1;
    let mut tot = 0;
    for line in lines {
        let line = line.unwrap();
        let (cycles, new_x) = if line == "noop" {
            (1, x)
        } else {
            let num = &line[5..];
            let num = num.parse::<i32>().unwrap();
            (2, x + num)
        };
        if report_i >= report_cycles.len() {
            break;
        }
        let rep = report_cycles[report_i];
        if cycle <= rep && cycle + cycles > rep {
            tot += x * rep;
            report_i += 1;
        };
        x = new_x;
        cycle += cycles;
    }
    println!("{}", tot);
}

pub fn part2() {
    let lines = read_lines("input10").unwrap();
    let mut x = 1;
    let mut crt_x = 0;
    for line in lines {
        let line = line.unwrap();
        let (cycles, new_x) = if line == "noop" {
            (1, x)
        } else {
            let num = &line[5..];
            let num = num.parse::<i32>().unwrap();
            (2, x + num)
        };
        for _ in 0..cycles {
            let ch = if (crt_x - x).abs() <= 1 {
                '#'
            } else {
                ' '
            };
            print!("{}", ch);
            crt_x += 1;
            if crt_x == 40 {
                crt_x = 0;
                println!();
            }
        }
        x = new_x;
    }
}
