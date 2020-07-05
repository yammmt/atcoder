// -*- coding:utf-8-unix -*-

// NOT WORK

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [String; h],
    }

    let mut total_black = 0;
    for i in &c {
        total_black += i.chars().filter(|&a| a == '#').count();
    }

    let mut ans = 0;
    for row_num in 0..h + 1 {
        // println!("h: {}", h);
        let mut current_black = total_black;
        let mut current_table = c.clone();
        for r in 0..row_num {
            // r 行の row を選ぶ
            // println!("row_num: {}", row_num);
            // println!("r: {}", r);
            let combinations = (0..h).combinations_with_replacement(r);
            // println!("{:?}", combinations);
            for comb in combinations {
                // 初期値
                let mut updated_table = c.clone();

                // 選ばれたところを塗り潰す
                for i in comb {
                    current_black -= updated_table[i].chars().filter(|&a| a == '#').count();
                    // 既に要求数未満であれば `continue`
                    if current_black < k {
                        continue;
                    }
                    updated_table[i] = updated_table[i].replace('#', &"r");
                }

                if current_black < k {
                    continue;
                }

                current_table = updated_table;
                // println!("before col_num");
                // println!("current_black: {}", current_black);
                for col_num in 0..w + 1 {
                    for c in 0..col_num {
                        let combinations = (0..w).combinations_with_replacement(c);
                        for comb in combinations {
                            // let mut updated_table = current_table.clone();

                            for i in comb {
                                let mut updated_black = current_black;
                                for j in &current_table {
                                    if j.as_bytes()[i] == b'#' {
                                        println!("k: {}", k);
                                        println!("updated_black: {}", updated_black);
                                        updated_black -= 1;
                                        if updated_black < k {
                                            continue;
                                        }
                                    }
                                    if updated_black < k {
                                        continue;
                                    }
                                }
                                current_black = updated_black;
                            }

                            if current_black == k {
                                ans += 1;
                            }
                        }
                    }
                }
                // println!("after col_num");
                // println!("ans: {}", ans);
            }
        }
    }
    println!("{}", ans);
}
