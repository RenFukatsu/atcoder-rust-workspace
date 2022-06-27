use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    let mut ans = Vec::<String>::new();
    if n % 2 == 0 {
        search(0, &n, 0, String::from(""), &mut ans);
    }
    for a in ans {
        println!("{}", a);
    }
}

fn search(i: i32, n: &i32, left: i32, s: String, ans: &mut Vec::<String>) {
    if i == *n {
        ans.push(s);
        return;
    }

    if left != *n - i {
        let mut sc = s.clone();
        sc.push('(');
        search(i + 1, n, left + 1, sc, ans);
    }
    if left > 0 {
        let mut sc = s.clone();
        sc.push(')');
        search(i + 1, n, left - 1, sc, ans);
    }
}