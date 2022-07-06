use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    let mut ans = 0;
    for i in 0..a+1 {
        for j in 0..b+1 {
            for k in 0..c+1 {
                if x == i * 500 + j * 100 + k * 50 {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
