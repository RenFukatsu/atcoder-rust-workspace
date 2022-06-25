use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    }

    let x_str = a + &b;
    let x = x_str.parse().unwrap();

    let mut ans = "No";
    for i in 0..1001 {
        if i * i == x {
            ans = "Yes";
        }
    }
    println!("{}", ans);
}
