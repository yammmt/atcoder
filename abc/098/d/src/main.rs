// サンプルは親切だがエッジケースが他にもあるからかペナ率がとても高い
// > 1h WA: 多分汚コードで分岐漏らした

use proconio::input;

fn add_ans(cur_l: usize, cur_r: usize, last_r: usize) -> usize {
    let duplicated_ans = if last_r > cur_l {
        (last_r - cur_l + 1) * (last_r - cur_l) / 2
    } else {
        0
    };
    (cur_r - cur_l + 1) * (cur_r - cur_l) / 2 - duplicated_ans
}

fn main() {
    input! {
        n: usize,
        an: [u64; n],
    }

    // 計算がめんどくさそうなので先に一要素分は数えておく
    let mut ans = n;
    let mut bit_num = vec![0; 20];
    let mut left_idx = 0;
    let mut last_r_idx = 0;
    for (i, a) in an.iter().enumerate() {
        // println!("{:>02}: {:>03}: {:>010b}", i, a, a);
        let mut cur_a = *a;
        let mut cur_i = 0;
        while cur_a > 0 {
            bit_num[cur_i] += cur_a % 2;
            cur_a /= 2;
            cur_i += 1;
        }

        if bit_num.iter().any(|&b| b > 1) {
            // 手前までの分でnC2
            ans += add_ans(left_idx, i - 1, last_r_idx);
            last_r_idx = i - 1;

            // 左側からしゃくとりで縮めていく
            while left_idx < i && bit_num.iter().any(|&b| b > 1) {
                let mut cur_a = an[left_idx];
                let mut cur_i = 0;
                while cur_a > 0 {
                    bit_num[cur_i] -= cur_a % 2;
                    cur_a /= 2;
                    cur_i += 1;
                }
                left_idx += 1;
            }
        }

        if i == n - 1 {
            // 最終回は更新する
            ans += add_ans(left_idx, i, last_r_idx);
        }
    }

    println!("{}", ans);
}
