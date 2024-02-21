use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: usize,
    }

    // n! 通り列挙して平方数判定を取ろうとすると TLE
    // とすると, 逆に平方数を列挙していって作れるか判定する方針を取りたい
    // 最長ケースが 10^14 -1 で, これ以下の数かつ平方数は sqrt を取って 10^7 -1 個だし間に合いそう

    let cnt_num = |mut m: usize| {
        let mut cnt = vec![0; 10];
        for _ in 0..n {
            cnt[m % 10] += 1;
            m /= 10;
        }
        while m > 0 {
            cnt[m % 10] += 1;
            m /= 10;
        }
        cnt
    };

    let num_cnt = cnt_num(s);

    let mut ans = 0;
    for i in 0..10_000_000 {
        let ii_cnt = cnt_num(i * i);
        if ii_cnt.iter().sum::<usize>() > n {
            break;
        }

        if num_cnt == ii_cnt {
            ans += 1;
        }
    }

    println!("{ans}");
}
