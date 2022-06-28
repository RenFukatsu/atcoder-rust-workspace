use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1],
    }
    let mut mp = vec![Vec::<usize>::new(); n];
    for (a, b) in ab {
        let a = a - 1;
        let b = b - 1;
        mp[a].push(b);
        mp[b].push(a);
    }
    let (node, _) = calc_dist(0, &mp);
    let (_, dist) = calc_dist(node, &mp);
    println!("{}", dist + 1);
}

fn calc_dist(p: usize, mp: &[Vec<usize>]) -> (usize, usize) {
    let mut que = VecDeque::new();
    que.push_back((p, p, 0_usize));
    let (mut point, mut dist) = (0_usize, 0_usize);
    while !que.is_empty() {
        let (mut pre, mut node, mut len) = (0_usize, 0_usize, 0_usize);
        match que.pop_front() {
            None => eprintln!("cannot pop queue"),
            Some((a, b, c)) => {
                pre = a;
                node = b;
                len = c;
            },
        }
        if dist < len {
            dist = len;
            point = node;
        }
        for next in &mp[node] {
            if pre == *next {
                continue;
            }
            que.push_back((node, *next, len + 1));
        }
    }
    (point, dist)
}