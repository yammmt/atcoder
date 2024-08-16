use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    // 350 点問題らしいが 500 点の間違いでは, 重実装...
    //   => 筋肉解法だったため

    // 1 桁の回文数は 0, ..., 9 で 10 個
    // 2 桁の回文数は 11, 22, ..., 99 で 9 個
    // 3 桁の回文数は 1{0, ..., 9}1, ... で 9x10=90 個
    // 4 桁の回文数は 先頭/末尾が 9 通りに間が 10 通りで 90 個
    // 5 桁の回文数は 先頭/末尾が 9 通りに 2/4 桁目が 10 通りと 3 桁目が 10 通りで 900 個
    // ... として, 1 桁目を除き, 回文数は桁数が奇数になる度に x10 される
    // 桁ごとに数え上げていく分には TLE しないが, 愚直数え上げでは例 3 のように TLE

    // 最上位桁が決まると, 自分より小さい回文数の総数が定まる.
    // これを利用し, 上位桁から順に, 自分より小さい回文数の総数を貪欲に減らしていくと
    // 解が一意に求まる (桁 DP の亜種？). が, 複雑である.

    if n < 11 {
        // 一桁 は 0 が使える
        println!("{}", n - 1);
        return;
    }

    let mut palindromic_num = 10;
    let mut digit_len = 2;
    // 最上位桁が 1 増えるごとにどれだけ回文数が増えるか
    let mut added_per_highest_digit = 1;
    while palindromic_num <= n {
        if digit_len % 2 == 1 {
            added_per_highest_digit *= 10;
        }

        // 最大ケースの 36 digits => 最上位桁が確定すると 34 桁残る
        // 34 digits 内の数の組み合わせは 10^17, 64 bit で足りる
        // 10^18 個目の回文数は 10^17 * 10
        if palindromic_num + added_per_highest_digit * 9 >= n {
            let mut ans = vec![0; digit_len];
            // 自分より小さい回文数の数
            let mut palindromic_num_left = n;
            // 最上位桁を固定すると回文数の個数が見積もれる
            for i in 1..=10 {
                // 最上位桁だけは 0 を使えないので処理を分ける
                let palindromic_num_cur = i * added_per_highest_digit;
                // = を忘れると 1WA
                if palindromic_num + palindromic_num_cur >= palindromic_num_left {
                    ans[0] = i;
                    ans[digit_len - 1] = i;
                    // `palindromic_num` は今より 1 桁小さいところまでの回文数の総数
                    // 桁数違い分は自分より必ず小さいので一気に引ける
                    palindromic_num_left -= palindromic_num + (i - 1) * added_per_highest_digit;
                    added_per_highest_digit /= 10;
                    break;
                }
            }

            // println!("left: {palindromic_num_left}");
            for i in 1..=digit_len / 2 {
                for j in 0..=10 {
                    // 最上位桁以外は 0 を含む
                    // 0 確定時に残りの回文数消費を正しく計算するため, +1 が所々に入る
                    let palindromic_num_cur = (j + 1) * added_per_highest_digit;
                    if palindromic_num_cur >= palindromic_num_left {
                        ans[i] = j;
                        ans[digit_len - 1 - i] = j;
                        // 自分より小さい回文数の個数を管理するので
                        // j=0 時に減算なし, j=1 時に 0x...x0 のパターン分全部引く, など
                        palindromic_num_left -= j * added_per_highest_digit;
                        added_per_highest_digit /= 10;
                        break;
                    }
                }
            }

            for a in ans {
                print!("{a}");
            }
            println!();
            return;
        }

        palindromic_num += added_per_highest_digit * 9;
        digit_len += 1;
    }
}
