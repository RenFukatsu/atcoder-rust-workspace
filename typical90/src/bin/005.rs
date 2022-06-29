use proconio::input;

fn main() {
    input! {
        n: usize,
        b: usize,
        k: usize,
        c: [usize; k],
    }
    let mut contain = [false; 10];
    for val in c {
        contain[val] = true;
    }
    let contain = contain;
    // eprintln!("contain = {:?}", contain);
    let modulo: usize = 1000000007;
    let mut ans = 0;
    let mut num = b;
    while num.to_string().len() < n {
        num += b;
    }
    while num.to_string().len() < n + 1 {
        let mut div = 1;
        for _ in 1..num.to_string().len() {
            div *= 10;
        }
        let mut ok = true;
        while div > 0 {
            let i = num % (div * 10) / div;
            if !contain[i] {
                ok = false;
            }
            div /= 10;
        }
        // eprintln!("num = {}, div = {}, ok = {}", num, div, ok);
        if ok {
            ans += 1;
        }
        num += b;
    }
    println!("{}", ans % modulo);
}
