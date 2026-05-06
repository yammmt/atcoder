use itertools::Itertools;
use proconio::fastout;
use proconio::input;

const TIME_MAX: usize = 200_001;

#[fastout]
fn main() {
    input! {
        n: usize,
        stn: [(usize, usize); n],
    }

    let mut time_in = Vec::with_capacity(n);
    let mut time_out = Vec::with_capacity(n);
    for i in 0..n {
        time_in.push(stn[i].0);
        time_out.push(stn[i].1);
    }
    time_in.sort_unstable();
    time_out.sort_unstable();

    // (累積和) 出社していない人の数を時間別に求める
    let mut not_appeared = [n; TIME_MAX];
    let mut time_in_idx = 0;
    let mut cur = n;
    for i in 0..TIME_MAX {
        while time_in_idx < n && time_in[time_in_idx] == i {
            cur -= 1;
            time_in_idx += 1;
        }
        not_appeared[i] = cur;
    }
    // (累積和) 退社した人の数を時間別に求める
    // 入退室時間重複時の計算の都合, 代入先は時間 1 だけ後にする
    let mut already_got_home = [0; TIME_MAX];
    let mut time_out_idx = 0;
    let mut cur = 0;
    for i in 0..TIME_MAX - 1 {
        while time_out_idx < n && time_out[time_out_idx] == i {
            cur += 1;
            time_out_idx += 1;
        }
        already_got_home[i + 1] = cur;
    }

    // 自身の分は先に引く
    let mut ans = vec![n - 1; n];
    for i in 0..n {
        ans[i] -= already_got_home[stn[i].0];
        ans[i] -= not_appeared[stn[i].1];
    }

    println!("{}", ans.iter().join(" "));
}
