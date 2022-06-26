use proconio::input;


fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let mut ans = 0;
    for i in 1..n+1 {
        let stri = i.to_string();
        let mut sum = 0;
        for c in stri.chars() {
            let num = c as i32 - '0' as i32;
            sum += num;
        }
        if a <= sum && sum <= b {
            ans += 1;
        }
    }
    println!("{}", ans);
}
