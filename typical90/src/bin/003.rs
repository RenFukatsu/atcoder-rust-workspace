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
    let mut ans = 0;
    search(0, 0, &mp, &mut ans);
    println!("{}", ans + 1);
}

fn search(pre: usize, now: usize, mp: &Vec<Vec<usize>>, ans: &mut i32) -> i32 {
    let mut mlen = 0;
    let mut m2len = 0;
    for next in &mp[now] {
        if pre == *next {
            continue;
        }
        let len = search(now, *next, mp, ans);
        if len > mlen {
            m2len = mlen;
            mlen = len;
        } else if len > m2len {
            m2len = len;
        }
    }
    *ans = std::cmp::max(*ans, mlen + m2len);
    mlen + 1
}
