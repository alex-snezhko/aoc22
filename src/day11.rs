#[derive(Clone)]
struct MonkeyBehavior {
    items: Vec<usize>,
    // get_next_val: Box<dyn Fn(usize) -> usize>,
    test_div: usize,
    get_next_val: (fn(usize, usize) -> usize, Option<usize>),
    if_true: usize,
    if_false: usize,
}

fn get_monkey_input(lines: &mut Lines<BufReader<File>>) -> MonkeyBehavior {
    lines.next();
    let starting = lines.next().unwrap().unwrap().strip_prefix("  Starting items: ").unwrap().split(", ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let operation = lines.next().unwrap().unwrap().strip_prefix("  Operation: new = old ").unwrap().to_owned();
    let by_val = &operation[2..];
    let op = match operation.chars().next().unwrap() {
        '*' => usize::wrapping_mul,
        '-' => usize::wrapping_sub,
        '+' => usize::wrapping_add,
        _ => usize::wrapping_div,
    };
    let other = match by_val {
        "old" => None,
        _ => Some(by_val.parse::<usize>().unwrap()),
    };
    let get_next_val = (op, other);
    let test_div = lines.next().unwrap().unwrap().strip_prefix("  Test: divisible by ").unwrap().parse::<usize>().unwrap();
    let if_true = lines.next().unwrap().unwrap().strip_prefix("    If true: throw to monkey ").unwrap().parse::<usize>().unwrap();
    let if_false = lines.next().unwrap().unwrap().strip_prefix("    If false: throw to monkey ").unwrap().parse::<usize>().unwrap();
    lines.next();
    MonkeyBehavior { items: starting, test_div, get_next_val, if_true, if_false }
}

pub fn part1() {
    let mut lines = read_lines("input11").unwrap();
    let mut monkey_behaviors = (0..8).map(|_| get_monkey_input(&mut lines)).collect::<Vec<_>>();
    let mut num_processed = vec![0; 8];
    for _ in 0..20 {
        for mbi in 0..monkey_behaviors.len() {
            let mb = monkey_behaviors[mbi].clone();
            for item in &mb.items {
                let (op, other) = mb.get_next_val;
                let after_inspection = match other {
                    None => op(*item, *item),
                    Some(other) => op(*item, other),
                };
                let next_val = after_inspection / 3;
                let pass_to = if next_val % mb.test_div == 0 { mb.if_true } else { mb.if_false };
                monkey_behaviors[pass_to].items.push(next_val);
                num_processed[mbi] += 1;
            }
            monkey_behaviors[mbi].items = vec![];
        }
    }
    println!("{:?}", num_processed);
    num_processed.sort();
    num_processed.reverse();
    println!("{}", num_processed[0] * num_processed[1]);
}

pub fn part2() {
    let mut lines = read_lines("input11").unwrap();
    let num = 8;
    let mut monkey_behaviors = (0..num).map(|_| get_monkey_input(&mut lines)).collect::<Vec<_>>();
    let lcd = monkey_behaviors.iter().fold(1, |x, y| x * y.test_div);
    let mut num_processed = vec![0; num];
    for _ in 0..10000 {
        for mbi in 0..monkey_behaviors.len() {
            let mb = monkey_behaviors[mbi].clone();
            for item in &mb.items {
                let (op, other) = mb.get_next_val;
                let next_val = match other {
                    None => op(*item, *item),
                    Some(other) => op(*item, other),
                };
                let next_val = next_val % lcd;
                let pass_to = if next_val % mb.test_div == 0 { mb.if_true } else { mb.if_false };
                monkey_behaviors[pass_to].items.push(next_val);
                num_processed[mbi] += 1;
            }
            monkey_behaviors[mbi].items = vec![];
        }
    }

    num_processed.sort();
    num_processed.reverse();
    let first = num_processed[0];
    let mut mult_digits = vec![];
    for (i, c) in num_processed[1].to_string().chars().rev().enumerate() {
        let n = c.to_digit(10).unwrap();
        mult_digits.push((first * n).to_string() + &"0".repeat(i));
    }

    let mut added = "0".repeat(mult_digits.iter().fold(0, |x, y| std::cmp::max(x, y.len())) + 1);
    for num in mult_digits {
        let mut i = added.len() - 1;
        for dig in num.chars().rev() {
            let dig = dig.to_digit(10).unwrap();
            let new_val = added.chars().nth(i).unwrap().to_digit(10).unwrap() + dig;
            let new_dig = new_val % 10;
            added = format!("{}{}{}", added.chars().take(i).collect::<String>(), new_dig, added.chars().skip(i + 1).collect::<String>());
            let mut overflow = new_val >= 10;
            let mut ofi = i - 1;
            while overflow {
                let new_val = added.chars().nth(ofi).unwrap().to_digit(10).unwrap() + 1;
                let new_dig = new_val % 10;
                added = format!("{}{}{}", added.chars().take(ofi).collect::<String>(), new_dig, added.chars().skip(ofi + 1).collect::<String>());
                overflow = new_val >= 10;
                ofi -= 1;
            }
            i -= 1;
        }
    }
    println!("{:?}", added);
    // wowie
}
