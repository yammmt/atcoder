// セグ木で殴れる
// WA: 掛け算の順序による切り捨て

use proconio::input;

#[allow(clippy::many_single_char_names)]
fn main() {
    input! {
        n: usize,
        mut cn: [u64; n], // > 0
        q: usize,
    }

    let mut odd_sold_num = 0;
    let mut all_sold_num = 0;
    let mut odd_sold_max = *(cn.iter().max().unwrap());
    let mut all_sold_max = *(cn.iter().min().unwrap());
    for (i, c) in cn.iter().enumerate() {
        if i % 2 == 0 {
            odd_sold_max = odd_sold_max.min(*c);
        }
    }

    let mut ans = 0u64;
    for _ in 0..q {
        input! {
            m: usize,
        }
        // println!("m: {}", m);
        match m {
            1 => {
                input! {
                    x: usize, //[1, N]
                    a: u64,
                }
                match x % 2 {
                    0 => {
                        if cn[x - 1] < all_sold_num + a {
                            continue;
                        }

                        cn[x - 1] -= a;
                        all_sold_max = all_sold_max.min(cn[x - 1] - all_sold_num);
                        ans += a;
                    }
                    _ => {
                        if cn[x - 1] < all_sold_num + odd_sold_num + a {
                            continue;
                        }

                        cn[x - 1] -= a;
                        let cnx1_left = cn[x - 1] - (all_sold_num + odd_sold_num);
                        odd_sold_max = odd_sold_max.min(cnx1_left);
                        all_sold_max = all_sold_max.min(cnx1_left);
                        ans += a;
                    }
                }
            }
            2 => {
                input! {
                    a: u64,
                }
                if a > odd_sold_max {
                    continue;
                }

                odd_sold_num += a;
                odd_sold_max -= a;
                all_sold_max = all_sold_max.min(odd_sold_max);
                ans += a * ((n as u64 + 1) / 2);
            }
            3 => {
                input! {
                    a: u64,
                }
                if a > all_sold_max {
                    continue;
                }

                all_sold_num += a;
                all_sold_max -= a;
                odd_sold_max -= a;
                ans += a * n as u64;
            }
            _ => unreachable!(),
        }
        // println!("ans: {}", ans);
    }

    println!("{}", ans);
}
