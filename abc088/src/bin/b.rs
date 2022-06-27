use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    a.sort_by_key(|&x| Reverse(x));

    let mut alice = 0;
    let mut bob = 0;
    for (i, val) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice += val;
        } else {
            bob += val;
        }
    }
    println!("{}", alice - bob);
}
