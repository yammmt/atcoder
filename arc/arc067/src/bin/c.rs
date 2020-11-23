use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let d = 10u64.pow(9) + 7;

    let mut prime_level = vec![0; 1001];
    // これ一括計算して高速化できないか
    for i in 2..n + 1 {
        let mut nn = i;
        let mut p = 2;
        while nn > 1 {
            if nn % p == 0 {
                prime_level[p as usize] += 1;
                nn /= p;
            } else {
                p += 1;
            }
        }
    }
    // println!("{:?}", prime_level);

    let mut ans = 1;
    for p in &prime_level {
        if *p == 0 {
            continue;
        }

        ans = (ans * (*p + 1)) % d;
    }
    println!("{}", ans);
}
