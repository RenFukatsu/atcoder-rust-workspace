use proconio::input;

const MOD: u32 = 1000000007;

fn main() {
    input! {
        n: usize,
        b: usize,
        k: usize,
        cs: [usize; k],
    }
    let mut rems = vec![0; b];
    for c in &cs {
        rems[c % b] += 1;
    }
    for _ in 0..n-1 {
        let mut nrems = vec![0; b];
        for (i, rem) in rems.iter().enumerate() {
            for c in &cs {
                let idx = (i * 10 + c) % b;
                nrems[idx] += *rem;
                nrems[idx] %= MOD;
            }
        }
        rems = nrems;
        // eprintln!("{:?}", rems);
    }
    println!("{}", rems[0]);
}
