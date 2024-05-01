use itertools::Itertools;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        abn: [(usize, usize); n],
    }

    // 並べる順番が 7 の順列で 5040 通り
    // 回転有無で 2^7 = 128 通り
    // `perm` に対するマスの操作が 7 通り
    //   この時点で 4,515,840 通りであり, 始点更新を全探索したり HashSet を使ったりすると TLE
    //   vec の move, clone とかも危険要素

    // 順に左上から置く
    for perm in (0..n).permutations(n) {
        // 反転の有無
        for bit in 0..2u32.pow(n as u32) {
            let mut vgrid = vec![vec![false; w]; h];
            let mut grid_fill_num = 0;
            let mut h_begin = 0;
            let mut w_begin = 0;
            'i_loop: for i in 0..n {
                let is_cur_turned = (bit >> i) % 2 == 0;

                if (is_cur_turned
                    && (h_begin + abn[perm[i]].0 > h || w_begin + abn[perm[i]].1 > w))
                    || (!is_cur_turned
                        && (h_begin + abn[perm[i]].1 > h || w_begin + abn[perm[i]].0 > w))
                {
                    // 範囲外, 次の回転パターンを試す
                    break;
                }

                for di in 0..abn[perm[i]].0 {
                    for dj in 0..abn[perm[i]].1 {
                        let i_cur = if is_cur_turned {
                            h_begin + di
                        } else {
                            h_begin + dj
                        };
                        let j_cur = if is_cur_turned {
                            w_begin + dj
                        } else {
                            w_begin + di
                        };
                        if vgrid[i_cur][j_cur] {
                            // 凸凹する場合にかかりうる
                            break 'i_loop;
                        }

                        vgrid[i_cur][j_cur] = true;
                        grid_fill_num += 1;
                    }
                }

                // 完成判定
                if grid_fill_num == h * w {
                    println!("Yes");
                    return;
                }

                // 始点を更新
                let w_nxt_candidate = if is_cur_turned {
                    // 回転なし
                    w_begin + abn[perm[i]].1
                } else {
                    // 回転あり
                    w_begin + abn[perm[i]].0
                };
                if w_nxt_candidate < w {
                    w_begin = w_nxt_candidate;
                } else {
                    // 始点と同じ行がすべて埋まった
                    'h_i_loop: for h_i in h_begin + 1..h {
                        for w_j in 0..w {
                            if !vgrid[h_i][w_j] {
                                h_begin = h_i;
                                w_begin = w_j;
                                break 'h_i_loop;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("No");
}
