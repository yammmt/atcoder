// :fu: 22-22 極端な重実装 ABC には多分出ないし出ても時間不足でかなりの Diff になりそう
// 図があっても問題文が読めず..

use proconio::input;
use proconio::marker::Chars;

// 転置して行順を逆にする
fn turn_left<T: Clone + Copy + Default>(sin: &[Vec<T>]) -> Vec<Vec<T>> {
    let h = sin.len();
    let w = sin[0].len();
    let mut ans = vec![vec![Default::default(); h]; w];
    for i in 0..w {
        for j in 0..h {
            ans[i][j] = sin[j][w - i - 1];
        }
    }
    ans
}

fn main() {
    input! {
        h: usize,
        w: usize,
        shw: [Chars; h],
        thw: [Chars; h],
    }

    // t の外周を切っておく
    let mut stamp_mass = vec![];
    for i in 0..h {
        for j in 0..w {
            if thw[i][j] == '#' {
                stamp_mass.push((i, j));
            }
        }
    }
    let mut th_min = h;
    let mut th_max = 0;
    let mut tw_min = w;
    let mut tw_max = 0;
    for m in &stamp_mass {
        th_min = th_min.min(m.0);
        th_max = th_max.max(m.0);
        tw_min = tw_min.min(m.1);
        tw_max = tw_max.max(m.1);
    }
    let mut newt = vec![vec![' '; tw_max - tw_min + 1]; th_max - th_min + 1];
    for i in th_min..=th_max {
        newt[i - th_min][0..(tw_max + 1 - tw_min)].clone_from_slice(&thw[i][tw_min..tw_max + 1])
    }
    let thw = newt;
    // println!("{:?}", thw);

    // 回転は全通り
    let t2 = turn_left(&thw);
    let t3 = turn_left(&t2);
    let t4 = turn_left(&t3);
    let vt = vec![thw, t2, t3, t4];
    // ok
    // for v in &vt {
    //     println!("{:?}", v);
    // }

    // 全 10x10 マスについて 90 度回転四回分を試しても 10^4 回で収まるので間に合う
    for sh in 0..h {
        for sw in 0..w {
            // println!("s: ({}, {})", sh, sw);
            for curt in &vt {
                let mut pass = true;
                // println!("  {:?}", curt);
                'outer: for th in 0..curt.len() {
                    for tw in 0..curt[0].len() {
                        let cursh = sh + th;
                        let cursw = sw + tw;

                        if cursh < h && cursw < w {
                            // "ハンコの底面は障害物が置かれているマスに重なっていない"
                            if shw[cursh][cursw] == '#' && curt[th][tw] == '#' {
                                // println!("    障害物");
                                pass = false;
                                break 'outer;
                            }
                        } else {
                            // "ハンコの底面はマス目からはみ出していない"
                            if curt[th][tw] == '#' {
                                // println!("    はみ出し");
                                pass = false;
                                break 'outer;
                            }
                        }
                    }
                }
                if pass {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
