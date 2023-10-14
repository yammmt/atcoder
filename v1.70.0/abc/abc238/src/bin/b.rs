// 苦手

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    // 回転方向を逆としても同じ
    let mut angles = vec![0];
    for a in &an {
        angles.push((angles.last().unwrap() + *a) % 360);
    }

    angles.sort_unstable();
    let mut ans = 0;
    for i in 0..angles.len() - 1 {
        ans = ans.max(angles[i + 1] - angles[i]);
    }
    // 最後に戻る分
    ans = ans.max(360 - angles.last().unwrap());

    println!("{ans}");
}
