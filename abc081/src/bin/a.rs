use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut sum = 0;
    for c in s.chars() {
        if c == '1' {
            sum += 1;
        }
    }
    println!("{}", sum);

}
