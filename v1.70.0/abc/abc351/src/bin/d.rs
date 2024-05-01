use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        shw: [Chars; h],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // 磁石なしマスはまんま連結成分を数えるだけ
    // 磁石ありマスに愚直に到達フラグを立てると WA
    //   磁石マスで行き止まりになるパターンは最大で 4 通りあるため,
    //   以下の入力で成分数が "4" とならなくなってしまう:
    // ...
    // #.#
    // ...
    //  =>  磁石に接するマスはその場で数えて戻る, とする
    //      重複判定のため, 何も考えず HashSet を使う (遅くなってそうだが)

    let has_magnet = |(i, j): (usize, usize)| {
        let mut ret = false;
        for d in &dir {
            let i_nxt = i.wrapping_add_signed(d.0);
            let j_nxt = j.wrapping_add_signed(d.1);
            if i_nxt < h && j_nxt < w && shw[i_nxt][j_nxt] == '#' {
                ret = true;
                break;
            }
        }
        ret
    };

    let mut ans = 1;
    let mut visited = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if visited[i][j] || shw[i][j] == '#' || has_magnet((i, j)) {
                continue;
            }

            // u32 で足りる
            let mut score = 0;
            let mut que = VecDeque::new();
            let mut counted_magnets = HashSet::new();
            que.push_back((i, j));
            // println!("{i} {j}");
            while let Some((i_cur, j_cur)) = que.pop_back() {
                let is_cur_magnet = has_magnet((i_cur, j_cur));
                if is_cur_magnet && !counted_magnets.contains(&(i_cur, j_cur)) {
                    // 今が動けないマスなら数えて戻る
                    // 到達済フラグも立てない
                    // println!("  {i_cur} {j_cur}");
                    score += 1;
                    counted_magnets.insert((i_cur, j_cur));
                    visited[i_cur][j_cur] = true;
                }
                if visited[i_cur][j_cur] {
                    continue;
                }

                visited[i_cur][j_cur] = true;
                score += 1;

                for d in &dir {
                    let i_nxt = i_cur.wrapping_add_signed(d.0);
                    let j_nxt = j_cur.wrapping_add_signed(d.1);
                    if i_nxt < h && j_nxt < w && shw[i_nxt][j_nxt] == '.' {
                        que.push_back((i_nxt, j_nxt));
                    }
                }
            }
            // println!("  {score}");
            ans = ans.max(score);
        }
    }

    println!("{ans}");
}
