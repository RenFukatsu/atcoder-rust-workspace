use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mp: [[usize; w]; h],
    }
    let mut sum_row = vec![0_usize; h];
    let mut sum_col = vec![0_usize; w];
    for (i, sr) in sum_row.iter_mut().enumerate() {
        for (j, sc) in sum_col.iter_mut().enumerate() {
            *sr += mp[i][j];
            *sc += mp[i][j];
        }
    }

    for (i, sr) in sum_row.iter_mut().enumerate() {
        for (j, sc) in sum_col.iter_mut().enumerate() {
            if j != 0 {
                print!(" ");
            }
            print!("{}", *sr + *sc - mp[i][j]);
        }
        println!();
    }
}
