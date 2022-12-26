struct Node {
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    val: i32,
}

fn part1() {
    let lines = read_lines("in20").unwrap().map(|x| x.unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();
    let n = lines.len();
    // let first = lines[0];
    // lines.into_iter().fold(Rc::new(RefCell::new(Node { left: None, right: None, val: first })), |prev_node, val| {
    //     let node = Rc::new(Node { left: Some(Rc::clone(&prev_node)), right: None, val });
    //     prev_node.right = Some(Rc::clone(&node));
    //     node
    // });
    // let positions = lines.iter().zip(0..n).collect::<HashMap<_, _>>();
    // let mut delta: Vec<i32> = [0].into_iter().cycle().take(n).collect();
    // let mut tot = 0;
    // for i in 0..n {
    //     println!("{:?}", lines);
    //     let actual_i = (i as i32 + delta[i]) as usize;
    //     let x = lines.remove(actual_i);
    //     let insert_i = (actual_i as i32 + x) as usize;
    //     lines.insert(insert_i, x);
    //     for j in 0..insert_i {
    //         delta[j] -= 1;
    //     }
    //     tot += 1;
    //     if tot == 5 {
    //         break;
    //     }
    // }
    // let mut i: i32 = 0;
    // while (i as usize) < n {
    //     println!("{} {:?}", i, lines);
    //     let x = lines.remove(i as usize);
    //     let new_i = (i as i32 + x).rem_euclid(n as i32) as usize;
    //     lines.insert(new_i, x);

    //     if (new_i as i32) >= i + 1 {
    //         i -= 1;
    //     }
    //     i += 1;
    // }
    // let zero_i = lines.iter().position(|x| *x == 0).unwrap();
    // println!("{}", lines[(zero_i + 1000) % n] + lines[(zero_i + 2000) % n] + lines[(zero_i + 3000) % n]);
}

fn part1() {
    let mut lines = read_lines("in20").unwrap().map(|x| x.unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();
    let n = lines.len();
    let mut i: i32 = 0;
    let mut tot = 0;
    while (i as usize) < n {
        println!("{} {:?}", i, lines);
        let x = lines.remove(i as usize);
        let new_i = (i as i32 + x).rem_euclid(n as i32) as usize;
        lines.insert(new_i, x);

        if (new_i as i32) >= i + 1 {
            i -= 1;
        }
        i += 1;

        tot += 1;
        if tot == 5 {
            break;
        }
    }
    let zero_i = lines.iter().position(|x| *x == 0).unwrap();
    println!("{}", lines[(zero_i + 1000) % n] + lines[(zero_i + 2000) % n] + lines[(zero_i + 3000) % n]);
}
