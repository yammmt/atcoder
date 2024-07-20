use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

const DUMMY: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        cn: [usize; n],
        q: usize,
    }

    // ちょっと実装が苦しい
    // 単品販売以外の販売数は一律管理できる
    // 在庫数少ない順の que をもつ…必要もなく, 在庫最小の品番だけ取れればよさそう
    // 0-origin の都合で問題文と偶数奇数が反転する

    let mut min_even = (0, DUMMY);
    let mut min_odd = (0, DUMMY);
    for i in 0..n {
        if i % 2 == 0 && cn[i] < min_even.1 {
            min_even = (i, cn[i]);
        } else if i % 2 == 1 && cn[i] < min_odd.1 {
            min_odd = (i, cn[i]);
        }
    }

    let mut sold_single = vec![0; n];
    let mut sold_even = 0;
    let mut sold_odd = 0;

    let mut ans = 0;
    for _ in 0..q {
        input! {
            e: usize,
        }
        match e {
            1 => {
                // バラ売り
                input! {
                    x: Usize1,
                    a: usize,
                }
                if x % 2 == 0 {
                    if sold_even + sold_single[x] + a <= cn[x] {
                        ans += a;
                        sold_single[x] += a;
                        if cn[x] - sold_single[x] < min_even.1 {
                            min_even = (x, cn[x] - sold_single[x]);
                        }
                    }
                } else {
                    if sold_odd + sold_single[x] + a <= cn[x] {
                        ans += a;
                        sold_single[x] += a;
                        if cn[x] - sold_single[x] < min_odd.1 {
                            min_odd = (x, cn[x] - sold_single[x]);
                        }
                    }
                }
            }
            2 => {
                // コード上は偶数売り
                input! {
                    a: usize,
                }
                if a + sold_even <= min_even.1 {
                    ans += a * ((n + 1) / 2);
                    sold_even += a;
                }
            }
            3 => {
                // 全種類売り
                input! {
                    a: usize,
                }
                if a + sold_even <= min_even.1 && a + sold_odd <= min_odd.1 {
                    ans += a * n;
                    sold_even += a;
                    sold_odd += a;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{ans}");
}
