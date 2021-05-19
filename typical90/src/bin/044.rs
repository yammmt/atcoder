use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut an: [i64; n],
        txyq: [(usize, usize, usize); q],
    }

    let mut cur_zero = 0;
    for txy in &txyq {
        match txy.0 {
            1 => an.swap((cur_zero + txy.1 - 1) % n, (cur_zero + txy.2 - 1) % n),
            2 => cur_zero = (cur_zero + n - 1) % n,
            3 => println!("{}", an[(cur_zero + txy.1 - 1) % n]),
            _ => unreachable!(),
        }
    }
}
