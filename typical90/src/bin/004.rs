use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mp: [[usize; w]; h],
    }
    let mut sum_row = vec![0_usize; h];
    let mut sum_col = vec![0_usize; w];
    for i in 0..h {
        for j in 0..w {
            sum_row[i] += mp[i][j];
            sum_col[j] += mp[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            if j != 0 {
                print!(" ");
            }
            print!("{}", sum_row[i] + sum_col[j] - mp[i][j]);
        }
        println!();
    }
}
