use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    for i in 0..1<<n {
        let mut s = String::new();
        for j in 0..n {
            if i & (1 << j) == 0 {
                s.push(')');
            } else {
                s.push('(');
            }
        }
        if is_correct(&s) {
            println!("{}", s);
        }
    }
}

fn is_correct(s: &String) -> bool {
    let mut x = 0;
    for c in s.chars() {
        if c == '(' {
            x += 1;
        } else {
            x -= 1;
        }
        if x < 0 {
            return false;
        }
    }
    if x != 0 {
        return false;
    }
    true
}