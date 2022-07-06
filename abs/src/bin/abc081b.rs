use proconio::input;

fn main() {
    input! {
        n: usize,
        mut alist: [usize; n],
    }

    let mut ans = 0;
    loop {
        let mut exist_odd = false;
        for a in alist.iter_mut() {
            if *a % 2 == 0 {
                *a /= 2;
            } else {
                exist_odd = true;
            }
        }
        if exist_odd {
            break;
        }
        ans += 1;
    }
    println!("{}", ans);
}
