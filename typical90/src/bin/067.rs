// 整数 N は 10 進数表記で 8^20 以下か質問投げたい (Yes だと思う)

// RE: N は 8 進数表記であり入力段階で 1e19 以上となり得る
// そもそも文字列で数値計算して茶色におさまるか？何か間違えてない？

use proconio::input;

fn main() {
    input! {
        // スマートではないが
        mut n: u128,
        k: usize,
    }

    for _ in 0..k {
        // 8 -> 10
        let mut ten = 0;
        let mut cur_eight = 1;
        while n > 0 {
            ten += n % 10 * cur_eight;
            n /= 10;
            cur_eight *= 8;
        }
        // println!("10: {}", ten);

        // 10 -> 9
        let mut nine = 0;
        let mut cur_nine = 1;
        while ten > 0 {
            nine += ten % 9 * cur_nine;
            ten /= 9;
            cur_nine *= 10;
        }
        // println!(" 9: {}", nine);

        // '8' -> '5'
        let mut eight = 0;
        let mut cur_ten = 1;
        while nine > 0 {
            eight += if nine % 10 == 8 {
                5 * cur_ten
            } else {
               nine % 10 * cur_ten
            };
            nine /= 10;
            cur_ten *= 10;
        }
        n = eight;
    }

    println!("{}", n);
}
