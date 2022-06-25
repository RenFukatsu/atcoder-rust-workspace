use proconio::input;

fn main() {
    input! {
        r: i8,
        g: i8,
        b: i8,
    }

    let x: i32 = r as i32 * 100 + g as i32 * 10 + b as i32;
    if x % 4 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
