use proconio::fastout;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DUMMY: usize = usize::MAX / 3;

fn cost_to(ahw: &Vec<Vec<usize>>, goal: (usize, usize)) -> Vec<Vec<usize>> {
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let h = ahw.len();
    let w = ahw[0].len();
    let mut ret = vec![vec![DUMMY; w]; h];
    let mut heap = BinaryHeap::new();

    heap.push(Reverse((0, goal)));
    while let Some(Reverse((cost_cur, v_cur))) = heap.pop() {
        if ret[v_cur.0][v_cur.1] != DUMMY {
            continue;
        }

        ret[v_cur.0][v_cur.1] = cost_cur;
        for d in &dir {
            let i_nxt = v_cur.0.wrapping_add_signed(d.0);
            let j_nxt = v_cur.1.wrapping_add_signed(d.1);
            if i_nxt < h && j_nxt < w {
                // 枝刈り入れても速度大差ないはず
                heap.push(Reverse((cost_cur + ahw[i_nxt][j_nxt], (i_nxt, j_nxt))));
            }
        }
    }

    ret
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[usize; w]; h],
    }

    // 中間地点を固定し、そこまで/そこから右下/そこから右上の整備コストを足す
    // 左下 -> 中間地点 -> 右下 -> 中間地点 -> 右上
    // とすると全点からこの 3 点までの最短コストを求めて愚直に足せばできるはず
    let to_lb = cost_to(&ahw, (h - 1, 0));
    let to_rb = cost_to(&ahw, (h - 1, w - 1));
    let to_rt = cost_to(&ahw, (0, w - 1));

    let mut ans = DUMMY;
    for i in 0..h {
        for j in 0..w {
            let cost = to_lb[i][j] + to_rb[i][j] + to_rt[i][j] - 2 * ahw[i][j];
            ans = ans.min(cost)
        }
    }

    println!("{ans}");
}
