use proconio::input;

fn main() {
    input! {
        n: usize,
        sn: [u64; n],
    }

    let mut ans = 0;
    for s in &sn {
        let mut pass = false;
        'outer: for a in 1..1001 {
            for b in 1..1001 {
                if 4 * a * b + 3 * a + 3 * b == *s {
                    pass = true;
                    break 'outer;
                }
            }
        }
        if !pass {
            ans += 1;
        }
    }

    println!("{}", ans);
}
