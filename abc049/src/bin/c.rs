use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let revs = s.chars().into_iter().rev().collect::<String>();
    let dre = ["dream", "dreamer", "erase", "eraser"];
    let revdre = dre.iter().map(|x| x.to_string().chars().rev().collect::<String>()).collect_vec();

    let mut i = 0;
    let mut ans = "YES";
    loop {
        let research = &revs[i..std::cmp::min(revs.len(), i+7)];
        let prei = i;
        for r in &revdre {
            if research.chars().next() == r.chars().next() && research.contains(r) {
                i += r.len();
                eprintln!("{}", r.chars().into_iter().rev().collect::<String>());
            }
        }
        if prei == i {
            ans = "NO";
            break;
        }
        if i >= revs.len() {
            break;
        }
    }
    println!("{}", ans);
}
