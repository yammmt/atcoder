use proconio::input;

const FAV_NUM: usize = 200;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    // j を固定し i を探索する方針

    // cnt[n]: これまでに出た n の個数
    let mut cnt = vec![0; FAV_NUM];
    let mut ans = 0i64;
    for a in &an {
        let cur = a % 200;
        ans += cnt[cur];
        cnt[cur] += 1;
    }

    println!("{ans}");
}
