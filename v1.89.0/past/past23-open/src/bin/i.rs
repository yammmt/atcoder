use proconio::fastout;
use proconio::input;
use proconio::marker::Bytes;

#[fastout]
fn main() {
    input! {
        n: usize,
        y: usize,
        sxn: [(Bytes, usize); n]
    }

    // 2^18 = 262,144 であり bit 全探索は間に合う
    // 長さ最大 100 かつ登場文字数は最大でも 26 でしかないので,
    // 先に数えて雑に最大値を覚えておくだけでも十分なはず

    let mut appeared = vec![vec![false; 26]; n];
    for i in 0..n {
        for &j in &sxn[i].0 {
            appeared[i][(j - b'a') as usize] = true;
        }
    }

    let mut ans = 0;
    for bit in 0..2usize.pow(n as u32) {
        let mut b = bit;
        let mut bought = [false; 26];
        let mut cost = 0;
        let mut i = 0;
        while b > 0 {
            if b & 1 == 1 {
                cost += sxn[i].1;
                for j in 0..26 {
                    if appeared[i][j] {
                        bought[j] = true;
                    }
                }
            }

            b /= 2;
            i += 1;
        }

        if cost > y {
            continue;
        }

        let score = bought.iter().filter(|&b| *b).count();
        ans = ans.max(score);
    }

    println!("{ans}");
}
