use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut alist: [usize; n],
    }

    alist.sort_by(|a, b| b.cmp(a));

    let mut alice = 0;
    let mut bob = 0;
    for (i, a) in alist.iter().enumerate() {
        if i % 2 == 0 {
            alice += a;
        } else {
            bob += a;
        }
    }

    println!("{}", alice - bob);
}
