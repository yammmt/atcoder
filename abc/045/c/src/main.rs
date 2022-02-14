// -> 7min

use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: Bytes,
    }
    let vs: Vec<u64> = s.iter().map(|&c| (c - b'0') as u64).collect();

    let mut ans = 0;

    for i in 0..2u64.pow((vs.len() - 1) as u32) {
        let mut cur_0 = 0;

        let mut cur_1 = 0;
        for (j, &n) in vs.iter().enumerate() {
            cur_1 *= 10;
            cur_1 += n;
            if (i >> j) & 0x01 == 1 {
                cur_0 += cur_1;
                cur_1 = 0;
            }
        }
        cur_0 += cur_1;

        ans += cur_0;
    }

    println!("{}", ans);
}
