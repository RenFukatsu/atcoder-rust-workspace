use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }

    let aval: i32 = 500;
    let bval: i32 = 100;
    let cval: i32 = 50;

    let mut st: HashSet<i32>;
    for i in 0..a {
        for j in 0..b {
            for k in 0..c {
                st.insert(aval * i + bval * j + cval * k);
            }
        }
    }

    println!("{}", st.len());
}
