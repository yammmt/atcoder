use proconio::input;

// 30min. 2WA

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut dn = Vec::with_capacity(n);
    for i in 0..n {
        dn.push(an[i] - (i as i64 + 1));
    }

    dn.sort();
    let mut ans = 0;
    for i in 0..n {
        ans += (an[i] - (dn[n / 2] + i as i64 + 1)).abs();
    }
    // 配列の長さが偶数時にずれた値も一応見たい気がするが見なくても AC だった
    // ex. n = 6 時の中央値は (an[2] + an[3]) / 2
    if n > 2 && n % 2 == 1 {
        let mut ans2 = 0;
        for i in 0..n {
            ans2 += (an[i] - (dn[n / 2 - 1] + i as i64 + 1)).abs();
        }
        ans = ans.min(ans2);
    }
    println!("{}", ans);
}
