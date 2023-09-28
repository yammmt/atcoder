// :fu: :fu: 21-03 インデックスの実装がややこしく嫌 PAST 後半っぽい

use proconio::input;
use proconio::marker::Chars;

fn bit_rows(n: u32) -> Vec<Vec<usize>> {
    let mut ret = vec![];
    for b in 0..2u64.pow(n) {
        let mut cur = vec![];
        for i in 0..n {
            if b & (1 << i) > 0 {
                cur.push(i as usize);
            }
        }
        ret.push(cur);
    }
    ret
}

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: i64,
        shw: [Chars; h],
    }

    let h_cut = bit_rows(h as u32 - 1);
    // println!("{:?}", h_cut);
    let mut ans = std::usize::MAX / 2;
    for br in h_cut {
        // 縦 i 個目の後に線を入れると考えるならば + 1 する
        let mut ranges = vec![];
        for b in &br {
            ranges.push(b + 1);
        }
        // 終点追加
        ranges.push(h);

        // 横線カットを全通り試す
        let mut pts = 0;
        let mut stored = vec![0; ranges.len()];
        let mut cur_stored = vec![0; ranges.len()];
        'j_loop: for j in 0..w {
            let mut h_from = 0;
            for (range_i, h_to) in ranges.iter().enumerate() {
                let mut col_sum = 0;
                for i in h_from..*h_to {
                    if shw[i][j] == '1' {
                        col_sum += 1;
                    }
                    // 縦線カットで補えないなら抜ける
                    if col_sum > k {
                        pts = std::usize::MAX / 2;
                        break 'j_loop;
                    }
                }
                if stored[range_i] + col_sum > k {
                    stored = vec![0; ranges.len()];
                    pts += 1;
                }

                cur_stored[range_i] = col_sum;
                h_from = *h_to;
            }

            for i in 0..stored.len() {
                stored[i] += cur_stored[i];
            }
        }
        ans = ans.min(pts + br.len());
    }

    println!("{}", ans);
}
