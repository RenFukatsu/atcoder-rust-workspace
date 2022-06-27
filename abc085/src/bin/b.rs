use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ds: [i32; n],
    }
    let st: HashSet<i32> = ds.into_iter().collect();
    println!("{}", st.len());
}
