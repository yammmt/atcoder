use proconio::fastout;
use proconio::input;

const DUMMY: isize = isize::MIN / 3;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    // TODO: DFS でも解けるらしい, あまり書いたことない解法だし書いてみる

    // ここが一番難しい
    let mut ann = vec![vec![DUMMY; n]; n];
    for i in 0..n {
        input! {
            a: [isize; n - i - 1],
        }
        for j in 0..n - i - 1 {
            ann[i][j + i + 1] = a[j];
            // この書き方だといらないはずだが
            ann[j + i + 1][i] = a[j];
        }
    }

    let mut ans = DUMMY;
    for bit in 0..3usize.pow(n as u32) {
        let mut members = vec![vec![]; 3];
        let mut bit_cur = bit;
        for i in 0..n {
            members[bit_cur % 3].push(i);
            bit_cur /= 3;
        }

        let mut score = 0;
        for member in &members {
            let l = member.len();
            for i in 0..l {
                for j in i + 1..l {
                    score += ann[member[i]][member[j]];
                }
            }
        }

        ans = ans.max(score);
    }

    println!("{ans}");
}
