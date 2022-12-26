use std::cell::RefCell;
use std::cmp::{min, max};
use std::collections::{VecDeque, HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1() {
    let vals = read_lines("input15").unwrap().map(|line| {
        let line = line.unwrap();
        let mut split = line.split(": closest beacon is at x=");
        let (sensor, beacon) = (split.next().unwrap(), split.next().unwrap());
        let mut sensor_split = sensor[12..].split(", y=").map(|x| x.parse::<i32>().unwrap());
        let (sx, sy) = (sensor_split.next().unwrap(), sensor_split.next().unwrap());
        let mut beacon_split = beacon.split(", y=").map(|x| x.parse::<i32>().unwrap());
        let (bx, by) = (beacon_split.next().unwrap(), beacon_split.next().unwrap());
        let dist = (sx - bx).abs() + (sy - by).abs();
        ((sx, sy), (bx, by), dist)
    }).collect::<Vec<_>>();
    let mut poses_marked = HashSet::new();
    let target = 2000000;
    for ((sx, sy), _, dist) in &vals {
        let y_dist = (target - sy).abs();
        if y_dist <= *dist {
            let num_pos = dist - y_dist;
            poses_marked.extend((sx - num_pos)..=(sx + num_pos));
        }
    }

    let to_exclude = vals
        .iter()
        .filter(|(_, (bx, by), _)| *by == target && poses_marked.contains(bx))
        .map(|(_, (bx, _), _)| bx)
        .collect::<HashSet<_>>()
        .len();
    let result = poses_marked.len() - to_exclude;
    println!("{}", result);
}

fn part2() {
    let vals = read_lines("input15").unwrap().map(|line| {
        let line = line.unwrap();
        let mut split = line.split(": closest beacon is at x=");
        let (sensor, beacon) = (split.next().unwrap(), split.next().unwrap());
        let mut sensor_split = sensor[12..].split(", y=").map(|x| x.parse::<i32>().unwrap());
        let (sx, sy) = (sensor_split.next().unwrap(), sensor_split.next().unwrap());
        let mut beacon_split = beacon.split(", y=").map(|x| x.parse::<i32>().unwrap());
        let (bx, by) = (beacon_split.next().unwrap(), beacon_split.next().unwrap());
        let dist = (sx - bx).abs() + (sy - by).abs();
        ((sx, sy), (bx, by), dist)
    }).collect::<Vec<_>>();
    let mut poses_marked = HashSet::new();
    let target = 2000000;
    for ((sx, sy), _, dist) in &vals {
        let y_dist = (target - sy).abs();
        if y_dist <= *dist {
            let num_pos = dist - y_dist;
            poses_marked.extend((sx - num_pos)..=(sx + num_pos));
        }
    }

    let to_exclude = vals
        .iter()
        .filter(|(_, (bx, by), _)| *by == target && poses_marked.contains(bx))
        .map(|(_, (bx, _), _)| bx)
        .collect::<HashSet<_>>()
        .len();
    let result = poses_marked.len() - to_exclude;
    println!("{}", result);
}

fn main() {
    part1();
    // part2();
}
