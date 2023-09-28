// :fu: 21-05 義務教育の範囲の数問に見えたがやたら苦しい
// WA: unsigned でオーバーフロー
// 制約より並走はあり得ない

use proconio::input;

#[allow(clippy::useless_let_if_seq)]
fn main() {
    input! {
        t2: (i64, i64),
        a2: (i64, i64),
        b2: (i64, i64),
    }

    let first_fast;
    let last_fast;
    if a2.0 > b2.0 || (a2.0 == b2.0 && a2.1 < b2.1) {
        first_fast = a2;
        last_fast = b2;
    } else {
        first_fast = b2;
        last_fast = a2;
    }

    // first が前半速く後半で last が抜き返すパターンに絞る
    let first_dist = t2.0 * (first_fast.0 - last_fast.0);
    let last_dist = t2.1 * (last_fast.1 - first_fast.1);
    if first_dist > last_dist {
        // 最初に抜かれてそれっきり
        println!("0");
        return;
    }

    let total_dist = last_dist - first_dist;
    if total_dist == 0 {
        // 一周しても元に戻るだけ
        println!("infinity");
        return;
    }

    let mut ans = 0;
    // first が追いつく回数
    ans += first_dist / total_dist;
    // last が追いつく回数
    ans *= 2;
    if first_dist % total_dist > 0 {
        // 端数が出るなら last が追いつく回数は first が追いつく回数より一回多い
        ans += 1;
    }

    println!("{}", ans);
}
