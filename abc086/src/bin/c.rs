use proconio::input;


fn main() {
    input! {
        n: usize,
        txys: [(i32, i32, i32); n],
    }

    let mut pret = 0;
    let mut prex = 0;
    let mut prey = 0;
    let mut ans = "Yes";
    for (t, x, y) in txys {
        let dt = t - pret;
        let dist = (x - prex).abs() + (y - prey).abs();
        if dt >= dist && (dt - dist) % 2 == 0 {
            pret = t;
            prex = x;
            prey = y;
            continue;
        }
        ans = "No";
        break;
    }

    println!("{}", ans);
}
