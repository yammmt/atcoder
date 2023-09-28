// :fu: 22-02 実装が変則的で辛い
// 緑らしいが観測範囲内の緑水あたり全然解けてない
// HTML 解説は実装については載せているだけ

// 使用済か否かを Vec で管理すると TLE 寸前
// 排他的論理和をペア成立直後に更新しても計算時間はたいして変わらず
// そもそもペア配列を VecDeque に渡す部分で clone が発生して低速になるわけで
// (あれば) ペアの片方と使用済 bit 列で管理したら 200ms 程度まで高速化された

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
    }
    let mut ann = vec![vec![0; 2 * n]; 2 * n];
    for i in 0..2 * n - 1 {
        input! {
            a_cur: [u64; 2 * n - (i + 1)],
        }

        for j in 0..a_cur.len() {
            ann[i][i + 1 + j] = a_cur[j];
            ann[i + 1 + j][i] = a_cur[j];
        }
    }
    let ann = ann;
    // println!("ann: {:?}", ann);

    // 参加者が最大 16 人であり順列全列挙 16! 通り試すでは TLE
    // 参加者を二つのグループに分けてグループ内でそれぞれ全通り並び替えるでは
    // 16C8 * 8! > 5.0 * 10^8 となりこれも実行時間 2s では TLE

    let mut completed_bit = 0u16;
    for i in 0..2 * n {
        completed_bit += 1 << i;
    }

    let mut ans = 0;
    // (直前のペア, 使用済なら bit 1, 今の点数)
    let mut vdq = VecDeque::new();
    vdq.push_back((Some(0), 1u16, 0));
    while let Some(cur) = vdq.pop_front() {
        'outer: for i in 0..2 * n {
            if (cur.1 >> i) & 0x01 == 1 {
                // 使用済
                continue;
            }

            let next_used = cur.1 | (1 << i);
            match cur.0 {
                Some(c0) => {
                    // ペア成立
                    let next_score = cur.2 ^ ann[c0][i];
                    if next_used == completed_bit {
                        ans = ans.max(next_score);
                    } else {
                        vdq.push_back((None, next_used, next_score));
                    }
                }
                None => {
                    // (0-indexed) 偶数番目選択時には最も若い番号のみを採用
                    vdq.push_back((Some(i), next_used, cur.2));
                    break 'outer;
                }
            }
        }
    }

    println!("{}", ans);
}
