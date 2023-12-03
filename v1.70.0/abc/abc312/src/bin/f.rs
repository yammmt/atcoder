use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        txn: [(usize, usize); n],
    }

    let mut opener = vec![];
    let mut wo_opener = vec![];
    let mut w_opener = vec![];
    for (t, x) in txn {
        match t {
            0 => wo_opener.push(x),
            1 => w_opener.push(x),
            2 => opener.push(x),
            _ => unreachable!(),
        }
    }
    // 満足所降順
    opener.sort_unstable();
    opener.reverse();
    wo_opener.sort_unstable();
    wo_opener.reverse();
    w_opener.sort_unstable();
    w_opener.reverse();

    // 缶切りで開けられるぶんだけ缶切りが必要な缶を購入し, あまりが缶切りが不要な缶
    // 累積和で N 個開けた際の価値を予め計算しておく　,
    // 順次計算もできるし定数倍高速化になるがバグらせそう
    // 範囲外判定が嫌だから上限個数超えた領域も埋めておく

    let cusum = |values: &Vec<_>, vcusum: &mut Vec<_>| {
        vcusum[0] = 0;
        for i in 1..=values.len() {
            vcusum[i] = vcusum[i - 1] + values[i - 1];
        }
        for i in values.len() + 1..vcusum.len() {
            vcusum[i] = vcusum[i - 1];
        }
    };

    let mut cusum_wo = vec![0; n + 1];
    cusum(&wo_opener, &mut cusum_wo);

    let mut cusum_w = vec![0; n + 1];
    cusum(&w_opener, &mut cusum_w);

    // 缶切りで開ける缶を i 個購入する
    // 缶切りで開けられるだけ開ける方法が正解とは限らない
    let mut ans = 0;
    let mut opener_num = 0;
    let mut openable_num = 0;
    for i in 0..=m.min(w_opener.len()) {
        if openable_num < i {
            opener_num += 1;
            if opener_num > m || opener_num > opener.len() {
                break;
            }

            openable_num += opener[opener_num - 1];
        }

        if opener_num + i > m {
            break;
        }

        ans = ans.max(cusum_w[i] + cusum_wo[m - i - opener_num]);
    }

    println!("{ans}");
}
