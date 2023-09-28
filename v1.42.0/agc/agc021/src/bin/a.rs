// 12min 遅い

use proconio::input;

fn score(mut n: i64) -> i64 {
    let mut ret = 0;
    while n > 0 {
        ret += n % 10;
        n /= 10;
    }
    ret
}

fn main() {
    input! {
        n: i64,
    }
    let mut ans = score(n);

    let mut digits = vec![];
    let mut nn = n;
    while nn > 0 {
        digits.push(nn % 10);
        nn /= 10;
    }
    digits.reverse();
    // println!("{:?}", digits);

    // 左から見て i 桁目から先を 9 にする
    for i in 0..digits.len() {
        // println!("i: {}", i);
        let mut cur = 0;
        for j in 0..i {
            // そのまま
            cur += digits[j as usize];
        }
        // 繰り下げ
        cur += digits[i as usize] - 1;
        // 残りは 9
        cur += (9 * (digits.len() - i - 1)) as i64;

        // println!("{}", cur);
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
