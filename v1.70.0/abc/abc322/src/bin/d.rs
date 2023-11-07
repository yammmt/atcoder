// TODO: マスを座標集合でもつ方針を試す

use proconio::input;
use proconio::marker::Chars;

fn turn_left(sin: &Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    let mut ans = vec![vec![' '; n]; n];
    for i in 0..n {
        for j in 0..n {
            ans[i][j] = sin[j][n - i - 1];
        }
    }
    ans
}

// `poly_begin` で示された座標を左上とする
fn add_poly(
    s_base: &Vec<Vec<char>>,
    s_added: &Vec<Vec<char>>,
    poly_begin: (isize, isize),
) -> Option<Vec<Vec<char>>> {
    let mut ret = s_base.clone();

    for i in 0..4 {
        let i_i = i as isize + poly_begin.0;
        for j in 0..4 {
            if s_added[i][j] == '.' {
                continue;
            }

            let j_i = j as isize + poly_begin.1;
            if !(0..4).contains(&i_i) || !(0..4).contains(&j_i) {
                if s_added[i][j] == '#' {
                    // ポリオミノが範囲外に出る
                    return None;
                } else {
                    continue;
                }
            }

            let i_u = i_i as usize;
            let j_u = j_i as usize;
            if s_base[i_u][j_u] == '#' {
                // ポリオミノが被る
                return None;
            }

            ret[i_u][j_u] = s_added[i][j];
        }
    }

    Some(ret)
}

fn main() {
    input! {
        p0: [Chars; 4],
        p1: [Chars; 4],
        p2: [Chars; 4],
    }

    // 平行移動の候補は最大で 4x4=16 per Polyomino
    // 回転は 4 通り
    // 合わせて 1 Polyomino に対し 16x4 = 64 通りの置き方がある
    // これが 3 つあると 64x3 = 192 通りにしかならず, 全探索ができそう
    // 実装はつらめ

    let p0_t0 = p0;
    let p0_t1 = turn_left(&p0_t0, 4);
    let p0_t2 = turn_left(&p0_t1, 4);
    let p0_t3 = turn_left(&p0_t2, 4);
    let vp0 = vec![p0_t0, p0_t1, p0_t2, p0_t3];

    let p1_t0 = p1;
    let p1_t1 = turn_left(&p1_t0, 4);
    let p1_t2 = turn_left(&p1_t1, 4);
    let p1_t3 = turn_left(&p1_t2, 4);
    let vp1 = vec![p1_t0, p1_t1, p1_t2, p1_t3];

    let p2_t0 = p2;
    let p2_t1 = turn_left(&p2_t0, 4);
    let p2_t2 = turn_left(&p2_t1, 4);
    let p2_t3 = turn_left(&p2_t2, 4);
    let vp2 = vec![p2_t0, p2_t1, p2_t2, p2_t3];

    let mut v = vec![];
    let mass_base = vec![vec!['.'; 4]; 4];
    for pp0 in vp0 {
        for i in -3..=3 {
            for j in -3..=3 {
                let Some(mass) = add_poly(&mass_base, &pp0, (i, j)) else { continue; };
                v.push(mass);
            }
        }
    }

    let mut vv = vec![];
    while let Some(mass) = v.pop() {
        for pp1 in &vp1 {
            for i in -3..=3 {
                for j in -3..=3 {
                    let Some(mass1) = add_poly(&mass, pp1, (i, j)) else { continue; };
                    vv.push(mass1);
                }
            }
        }
    }

    while let Some(mass) = vv.pop() {
        for pp2 in &vp2 {
            for i in -3..=3 {
                for j in -3..=3 {
                    let Some(mass2) = add_poly(&mass, pp2, (i, j)) else { continue; };
                    let mut sharp_num = 0;
                    for ii in 0..4 {
                        for jj in 0..4 {
                            if mass2[ii][jj] == '#' {
                                sharp_num += 1;
                            }
                        }
                    }
                    if sharp_num == 16 {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    // for item in &v {
    //     for i in 0..4 {
    //         println!("{:?}", item[i]);
    //     }
    //     println!();
    // }
    // return;

    println!("No");
}
