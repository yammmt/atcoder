use proconio::input;

fn main() {
    input! {
        n: usize,
        cpn: [(usize, i64); n],
        q: usize,
        lrq: [(usize, usize); q],
    }

    let mut pts = vec![vec![0; 100_001]; 2];
    for (i, cp) in cpn.iter().enumerate() {
        pts[cp.0 - 1][i + 1] = cp.1;
    }

    let mut pts_sum = vec![vec![0; 100_001]; 2];
    for i in 0..2 {
        for j in 1..100_001 {
            pts_sum[i][j] = pts_sum[i][j - 1] + pts[i][j];
        }
    }

    for lr in &lrq {
        println!(
            "{} {}",
            pts_sum[0][lr.1] - pts_sum[0][lr.0 - 1],
            pts_sum[1][lr.1] - pts_sum[1][lr.0 - 1]
        );
    }
}
