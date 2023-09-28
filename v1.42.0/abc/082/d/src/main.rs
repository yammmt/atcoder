// 部分和 DP
// ペナ率も正解人数もエグい 細かな配慮が重め
// TLE: 愚直

use proconio::input;
use proconio::marker::Chars;

const ARRAY_SIZE: usize = 16002;
const AXIS_OFFSET: usize = 8000;

// debug
// const ARRAY_SIZE: usize = 22;
// const AXIS_OFFSET: usize = 10;

fn could_arrive(move_array: &[isize], goal: isize) -> bool {
    let move_sum = move_array.iter().sum::<isize>();
    let mut dp = vec![vec![false; ARRAY_SIZE]; move_array.len() + 1];
    dp[0][AXIS_OFFSET] = true;
    for (i, m) in move_array.iter().enumerate() {
        for j in 0..ARRAY_SIZE {
            if !dp[i][j] {
                continue;
            }

            dp[i + 1][j] = true;
            dp[i + 1][j + *m as usize] = true;
        }
    }

    let mut pass = false;
    for i in 0..ARRAY_SIZE {
        if !dp[move_array.len()][i] {
            continue;
        }

        // 怪しい
        let could = i - (move_sum as usize - (i - AXIS_OFFSET));
        let fixed_goal = (goal + AXIS_OFFSET as isize) as usize;
        if could == fixed_goal {
            pass = true;
            break;
        }
    }

    pass
}

fn main() {
    input! {
        s: Chars,
        x: isize,
        y: isize,
    }

    let mut x_move = vec![];
    let mut y_move = vec![];
    let mut first_dir = true;
    let mut x_offset = 0;
    let mut is_x = true;
    let mut cur = 0;
    for c in &s {
        if *c == 'F' {
            if first_dir {
                x_offset += 1;
            } else {
                cur += 1;
            }
        } else {
            if !first_dir {
                if is_x {
                    x_move.push(cur);
                } else {
                    y_move.push(cur);
                }
            }
            first_dir = false;
            is_x = !is_x;
            cur = 0;
        }
    }
    if s[s.len() - 1] == 'F' {
        if is_x {
            x_move.push(cur);
        } else {
            y_move.push(cur);
        }
    }
    // println!("{:?}", x_move);
    // println!("{:?}", y_move);

    // 部分和 DP を各軸に
    println!(
        "{}",
        if could_arrive(&x_move, x - x_offset) && could_arrive(&y_move, y) {
            "Yes"
        } else {
            "No"
        }
    );
}
