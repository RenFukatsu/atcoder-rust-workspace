use proconio::input;

fn main() {
    input! {
        n: i32,
        l: i32,
        k: i32,
        mut alist: [i32; n],
    }
    alist.push(l);
    let mut ng = l;
    let mut ok = 1;
    while ng - ok > 1 {
        let x = (ng + ok) / 2;
        let mut start = 0;
        let mut count = 0;
        for a in &alist {
            if a - start >= x {
                count += 1;
                start = *a;
            }
        }
        if count > k {
            ok = x;
        } else {
            ng = x;
        }
    }
    println!("{}", ok);
}
