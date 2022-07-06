use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let mut ans = 0;
    for c in n.chars() {
        if c == '1' {
            ans += 1;
        }
    }

    println!("{}", ans);
}
