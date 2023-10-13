use proconio::input;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    }

    // (x1, y1) からの距離が sqrt(5) となる点に対し (x2, y2) からの距離を測る
    let dir = [
        (-2, -1),
        (-1, -2),
        (-2, 1),
        (-1, 2),
        (1, -2),
        (2, -1),
        (1, 2),
        (2, 1),
    ];
    for d in &dir {
        let xc = x1 + d.0;
        let yc = y1 + d.1;
        let dist_sq = (x2 - xc).pow(2) + (y2 - yc).pow(2);
        if dist_sq == 5 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
