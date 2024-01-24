use proconio::fastout;
use proconio::input;
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    // 線形リストで好きなところに代入を, とすると同じ数字を大量に並べられると辛そう
    // k は高々 5 として愚直にしようにも代入を続けられると TLE
    // k 以外に成約の緩いところはないし BTreeSet で取るしかない？珍し

    let mut bts = BTreeSet::new();
    for i in 0..q {
        input! {
            c: usize,
            x: usize,
        }
        match c {
            1 => {
                bts.insert((x, i + 1));
            }
            2 => {
                input! {
                    k: usize,
                }
                // x 以下で大きい方から k 番目
                let mut cur = Some((x, usize::MAX));
                for _ in 0..k {
                    if cur.is_none() {
                        break;
                    }

                    let c_u = cur.unwrap();
                    cur = bts.range(..=(c_u.0, c_u.1 - 1)).next_back().copied();
                }
                if cur.is_none() {
                    println!("-1");
                } else {
                    println!("{}", cur.unwrap().0);
                }
            }
            3 => {
                input! {
                    k: usize,
                }
                // x 以上で小さい方から k 番目
                let mut cur = Some((x, 0));
                for _ in 0..k {
                    if cur.is_none() {
                        break;
                    }

                    let c_u = cur.unwrap();
                    cur = bts.range((c_u.0, c_u.1 + 1)..).next().copied();
                }
                if cur.is_none() {
                    println!("-1");
                } else {
                    println!("{}", cur.unwrap().0);
                }
            }
            _ => unreachable!(),
        }
    }
}
