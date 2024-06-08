use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        at: [Usize1; t],
    }
    let nn = n * n;

    // 随時書かれたマスを含むパターンだけを判定するともう少し高速になる

    let mut bingo = vec![None; nn];
    for (i, &a) in at.iter().enumerate() {
        bingo[a] = Some(i + 1);
    }

    let mut goal_turn: Option<usize> = None;
    let mut update_ans = |turn| {
        if let Some(gt) = goal_turn {
            goal_turn = Some(gt.min(turn));
        } else {
            goal_turn = Some(turn);
        }
    };

    // 横
    let mut i = 0;
    while i < nn {
        let mut goal_h = true;
        let mut max_turn = 0;
        for j in 0..n {
            let Some(bt) = bingo[i + j] else { goal_h = false; break; };
            max_turn = max_turn.max(bt);
        }

        if goal_h {
            update_ans(max_turn);
        }
        i += n;
    }

    // 縦
    for i in 0..n {
        let mut goal_v = true;
        let mut max_turn = 0;
        for j in 0..n {
            let Some(bt) = bingo[i + j * n] else { goal_v = false; break; };
            max_turn = max_turn.max(bt);
        }

        if goal_v {
            update_ans(max_turn);
        }
    }

    // -> 右下
    let mut goal_rb = true;
    let mut max_turn = 0;
    for j in 0..n {
        let Some(bt) = bingo[j * n + j] else { goal_rb = false; break; };
        max_turn = max_turn.max(bt);
    }

    if goal_rb {
        update_ans(max_turn);
    }

    // -> 左下
    let mut goal_lb = true;
    let mut max_turn = 0;
    for j in 0..n {
        let Some(bt) = bingo[n - 1 + j * n - j] else { goal_lb = false; break; };
        max_turn = max_turn.max(bt);
    }

    if goal_lb {
        update_ans(max_turn);
    }

    if let Some(ans) = goal_turn {
        println!("{ans}");
    } else {
        println!("-1");
    }
}
