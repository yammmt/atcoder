// :fu: 21-07 実装も得意でない

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        xyzwq: [(usize, usize, usize, usize); q],
    }
    let d = 1_000_000_007;

    // N <= 12 より,各桁の {0, 1} 選択を全列挙して高々 2^12 通り
    // 桁単位で OR 条件に対して条件を満たすかを考える

    let mut ptrn = vec![0; 60];
    for i in 0..60 {
        let mut cur = 0;
        for j in 0..2_u32.pow(n as u32) {
            let mut bit_row = vec![];
            for k in 0..n {
                bit_row.push((j >> k) & 1);
            }

            let mut pass = true;
            for xyzw in &xyzwq {
                if bit_row[xyzw.0 - 1] | bit_row[xyzw.1 - 1] | bit_row[xyzw.2 - 1] != ((xyzw.3 >> i) & 1) as u32
                {
                    pass = false;
                    break;
                }
            }

            if pass {
                cur += 1;
            }
        }
        ptrn[i] = cur;
    }

    let mut ans = 1_u64;
    for p in &ptrn {
        ans = (ans * *p) % d;
    }

    println!("{}", ans);
}
