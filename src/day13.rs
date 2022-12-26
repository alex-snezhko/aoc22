enum Token {
    LBrack,
    RBrack,
    Num(i32),
}

#[derive(Debug, Clone, PartialEq)]
enum Val {
    Num(i32),
    List(Vec<Val>)
}

fn tokenize(line: &str) -> Vec<Token> {
    let mut iter = line.chars().peekable();
    let mut tokens = vec![];
    while iter.peek().is_some() {
        let c = iter.next().unwrap();
        match c {
            '[' => tokens.push(Token::LBrack),
            ']' => tokens.push(Token::RBrack),
            ',' => (),
            _ => {
                let mut chars = vec![c];
                while iter.peek().unwrap().is_digit(10) {
                    chars.push(iter.next().unwrap());
                }
                if *iter.peek().unwrap() == ',' {
                    iter.next();
                }
                let num = chars.iter().map(|c| c.to_string()).collect::<Vec<_>>().join("").parse::<i32>().unwrap();
                tokens.push(Token::Num(num))
            }
        };
    }
    tokens
}

fn parse(iter: &mut Peekable<Iter<Token>>) -> Vec<Val> {
    let mut list = vec![];
    while iter.peek().is_some() {
        let tok = iter.next().unwrap();
        list.push(match tok {
            Token::LBrack => Val::List(parse(iter)),
            Token::RBrack => return list,
            Token::Num(num) => Val::Num(*num)
        });
    }
    list
}

fn parse_line(line: &str) -> Vec<Val> {
    let tokens = tokenize(line);
    let mut iter = tokens.iter().peekable();
    parse(&mut iter)
}

fn compare(left: &Vec<Val>, right: &Vec<Val>) -> Ordering {
    let mut liter = left.iter();
    let mut riter = right.iter();

    let mut ord = Ordering::Equal;
    while ord == Ordering::Equal {
        ord = match (liter.next(), riter.next()) {
            (Some(l), Some(r)) => {
                match (l, r) {
                    (Val::Num(ln), Val::Num(rn)) => ln.cmp(&rn),
                    (Val::Num(_), Val::List(rl)) => compare(&vec![l.clone()], &rl),
                    (Val::List(ll), Val::Num(_)) => compare(&ll, &vec![r.clone()]),
                    (Val::List(ll), Val::List(rl)) => compare(&ll, &rl),
                }
            },
            (None, None) => return Ordering::Equal,
            (None, _) => Ordering::Less,
            (_, None) => Ordering::Greater,
        };
    };
    ord
}

pub fn part1() {
    let mut lines = read_lines("input13").unwrap().peekable();
    let mut i = 1;
    let mut indices = vec![];
    while lines.peek().is_some() {
        let left = lines.next().unwrap().unwrap();
        let right = lines.next().unwrap().unwrap();
        let left = parse_line(&left);
        let right = parse_line(&right);
        let ord = compare(&left, &right);
        if ord == Ordering::Less {
            indices.push(i);
        }
        lines.next();
        i += 1;
    }
    let result: i32 = indices.iter().sum();
    println!("{}", result);
}

pub fn part2() {
    let mut lines = read_lines("input13").unwrap().peekable();
    let mut all_lines = vec![];
    while lines.peek().is_some() {
        let left = lines.next().unwrap().unwrap();
        let right = lines.next().unwrap().unwrap();
        let left = parse_line(&left);
        let right = parse_line(&right);
        all_lines.push(left);
        all_lines.push(right);
        lines.next();
    }
    let val1 = vec![Val::List(vec![Val::Num(2)])];
    let v1 = val1.clone();
    let val2 = vec![Val::List(vec![Val::Num(6)])];
    let v2 = val2.clone();
    all_lines.push(val1);
    all_lines.push(val2);
    all_lines.sort_by(|a, b| compare(a, b));
    let i1 = all_lines.iter().position(|x| *x == v1).unwrap() + 1;
    let i2 = all_lines.iter().position(|x| *x == v2).unwrap() + 1;
    println!("{}", i1 * i2);
}
