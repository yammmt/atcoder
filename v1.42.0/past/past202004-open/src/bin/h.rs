use proconio::input;
use proconio::marker::Chars;

const UNVISITED: i64 = std::i64::MAX / 3;

fn main() {
    input! {
        n: usize,
        m: usize,
        anm: [Chars; n],
    }

    let mut vertexes = vec![vec![]; 11];
    for i in 0..n {
        for j in 0..m {
            let level = match anm[i][j] {
                'S' => 0,
                'G' => 10,
                b => (b as u8 - b'0') as usize,
            };
            vertexes[level].push((i, j));
        }
    }
    let sx = vertexes[0][0].0;
    let sy = vertexes[0][0].1;
    let gx = vertexes[10][0].0;
    let gy = vertexes[10][0].1;

    let mut cost = vec![vec![UNVISITED; m]; n];
    cost[sx][sy] = 0;
    for i in 1..11 {
        let mut cur = vec![vec![UNVISITED; m]; n];
        for cur_g in &vertexes[i] {
            let cur_gx = cur_g.0;
            let cur_gy = cur_g.1;
            for prev_g in &vertexes[i - 1] {
                let prev_gx = prev_g.0;
                let prev_gy = prev_g.1;
                cur[cur_gx][cur_gy] = cur[cur_gx][cur_gy].min(
                    cost[prev_gx][prev_gy]
                        + (cur_gx as i64 - prev_gx as i64).abs()
                        + (cur_gy as i64 - prev_gy as i64).abs()
                );
            }
        }
        cost = cur;
    }

    println!(
        "{}",
        if cost[gx][gy] == UNVISITED {
            -1
        } else {
            cost[gx][gy]
        }
    );
}
