use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

const DUMMY: usize = usize::MAX / 3;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        snm_c: [Chars; n],
    }

    let mut snm = vec![];
    for i in 0..n {
        let mut v = vec![];
        for j in 0..m {
            v.push((snm_c[i][j] as u8 - b'0') as usize);
        }
        snm.push(v);
    }

    // 1x1 なら条件を満たすので最小値は 1, 0 でも誤差だが
    let mut ans = 1;
    for i in 0..n {
        for j in 0..m {
            for d in 0..n.min(m) {
                let i_max = i + d;
                let j_max = j + d;
                if i_max >= n || j_max >= m {
                    break;
                }

                // map で重複取ってもよいが, 種類数がたいしたことないので
                // vector を使った方が高速に動きそう
                let mut cnt = vec![0; 10];
                for di in 0..=d {
                    let i_cur = i + di;
                    for dj in 0..=d {
                        let j_cur = j + dj;
                        cnt[snm[i_cur][j_cur]] += 1;
                    }
                }
                // println!("{i} {j} {d}");
                // println!("  {:?}", cnt);

                let mut k_cur = DUMMY;
                let square_num = (d + 1) * (d + 1);
                for c in cnt {
                    k_cur = k_cur.min(square_num - c);
                }

                if k_cur <= k {
                    ans = ans.max(d + 1);
                }
            }
        }
    }

    println!("{ans}");
}
