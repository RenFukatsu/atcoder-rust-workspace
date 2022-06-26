use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    }

    let aval: i32 = 500;
    let bval: i32 = 100;
    let cval: i32 = 50;

    let mut ans = 0;
    for i in 0..a+1 {
        for j in 0..b+1 {
            for k in 0..c+1 {
                if aval * i + bval * j + cval * k == x {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
