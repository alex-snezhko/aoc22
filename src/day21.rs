struct MonkeyInfo<'a> {
    first: &'a str,
    second: &'a str,
    op: fn(i64, i64) -> i64,
}

fn get_monkey_value<'a>(name: &'a str, values: &mut HashMap<&'a str, i64>, infos: &HashMap<&'a str, MonkeyInfo<'a>>) -> i64 {
    match values.get(name) {
        Some(val) => *val,
        None => {
            let info = infos.get(name).unwrap();
            let first = get_monkey_value(info.first, values, infos);
            let second = get_monkey_value(info.second, values, infos);
            let val = (info.op)(first, second);
            values.insert(name, val);
            val
        },
    }
}

fn part1() {
    let lines = read_lines("input21").unwrap().map(|x| x.unwrap()).collect::<Vec<_>>();
    // let mut indices = HashMap::new();
    let mut values = HashMap::new();
    let mut infos = HashMap::new();
    for line in &lines {
        let name = &line[0..4];
        let action = &line[6..];
        match action.parse::<i64>() {
            Ok(num) => { values.insert(name, num); },
            _ => {
                let first = &action[0..4];
                let second = &action[7..];
                let op = match action.chars().nth(5).unwrap() {
                    '+' => i64::wrapping_add,
                    '-' => i64::wrapping_sub,
                    '*' => i64::wrapping_mul,
                    '/' =>  i64::wrapping_div,
                    _ => panic!()
                };
                let info = MonkeyInfo { first, second, op };
                infos.insert(name, info);
            }
        }
    }
    println!("{:?}", values);
    let root_val = get_monkey_value("root", &mut values, &infos);
    println!("{}", root_val);
}

struct MonkeyInfo2<'a> {
    first: &'a str,
    second: &'a str,
    op: char
}

#[derive(Debug)]
enum Thing {
    Human(Vec<(char, i64)>),
    Value(i64),
}

fn get_monkey_value2<'a>(name: &'a str, values: &mut HashMap<&'a str, Rc<Thing>>, infos: &HashMap<&'a str, MonkeyInfo2<'a>>) -> Rc<Thing> {
    match values.get(name) {
        Some(val) => Rc::clone(val),
        None => {
            if name == "humn" {
                Rc::new(Thing::Human(vec![]))
            } else {
                let info = infos.get(name).unwrap();
                let first = get_monkey_value2(info.first, values, infos);
                let second = get_monkey_value2(info.second, values, infos);
                let val = match (&*first, &*second) {
                    (Thing::Human(h), Thing::Value(v)) | (Thing::Value(v), Thing::Human(h)) => {
                        let mut copy = h.clone();
                        copy.push((info.op, *v));
                        Thing::Human(copy)
                    },
                    (Thing::Value(v1), Thing::Value(v2)) => {
                        let op = match info.op {
                            '+' => i64::wrapping_add,
                            '-' => i64::wrapping_sub,
                            '*' => i64::wrapping_mul,
                            '/' =>  i64::wrapping_div,
                            _ => panic!()
                        };
                        Thing::Value(op(*v1, *v2))
                    },
                    _ => panic!("whoops")
                };
                let val = Rc::new(val);
                values.insert(name, Rc::clone(&val));
                val
            }
        },
    }
}

fn part2() {
    let lines = read_lines("in21").unwrap().map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut values = HashMap::new();
    let mut infos = HashMap::new();
    for line in &lines {
        let name = &line[0..4];
        if name != "humn" {
            let action = &line[6..];
            match action.parse::<i64>() {
                Ok(num) if name != "humn" => { values.insert(name, Rc::new(Thing::Value(num))); },
                _ => {
                    let first = &action[0..4];
                    let second = &action[7..];
                    let op = action.chars().nth(5).unwrap();
                    let info = MonkeyInfo2 { first, second, op };
                    infos.insert(name, info);
                }
            }
        }
    }
    let root = infos.get("root").unwrap();

    let first_val = get_monkey_value2(root.first, &mut values, &infos);
    let second_val = get_monkey_value2(root.second, &mut values, &infos);
    let target = match (&*first_val, &*second_val) {
        (Thing::Human(h), Thing::Value(v)) | (Thing::Value(v), Thing::Human(h)) => {
            println!("{:?}", h);
            h
                .iter()
                .rev()
                .fold(*v, |val, (op, first_arg)| {
                    let actual_op = match op {
                        '+' => i64::wrapping_sub,
                        '-' => i64::wrapping_add,
                        '*' => i64::wrapping_div,
                        '/' =>  i64::wrapping_mul,
                        _ => panic!("op impossible")
                    };
                    actual_op(val, *first_arg)
                })
        },
        _ => panic!("No human branch")
    };
    println!("{}", target);
}
