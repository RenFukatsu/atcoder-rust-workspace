use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32,
    }
    let mut ans = (-1, -1, -1);
    for i in 0..n + 1 {
        for j in 0..n - i + 1 {
            let k = n - i - j;
            if i * 10000 + j * 5000 + k * 1000 == y {
                ans = (i, j, k);
            }
        }
    }
    println!("{} {} {}", ans.0, ans.1, ans.2);
}
