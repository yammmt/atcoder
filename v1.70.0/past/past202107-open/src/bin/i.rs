use proconio::input;

fn main() {
    input! {
        n: usize,
        xy2: [(f64, f64); 2],
        abn: [(f64, f64); n],
    }

    // 両目の y 座標を一致させ, かつ右 < 左となるまで回転する
    // 回転後の残り分は平行移動

    // 右目を基準に左目を回して回転角度を計算する
    // ようわからんからとりあえず二分探索してみる
    let mut deg_min = 0.0f64;
    let mut deg_max = 360.0f64;
    let mut cnt = 0;
    while (deg_max - deg_min) > 1e-10 && cnt < 100_000 {
        let mid = (deg_max + deg_min) / 2.0;
        let cos_r = (mid * std::f64::consts::PI / 180.0).cos();
        let sin_r = (mid * std::f64::consts::PI / 180.0).sin();

        let y0 = xy2[0].0 * sin_r + xy2[0].1 * cos_r;
        let y1 = xy2[1].0 * sin_r + xy2[1].1 * cos_r;

        if y1 - y0 > 0.0 {
            deg_max = mid;
        } else {
            deg_min = mid;
        }
        cnt += 1;
    }
    // println!("deg: {deg_max} {deg_min}");

    let cos_r = (deg_max * std::f64::consts::PI / 180.0).cos();
    let sin_r = (deg_max * std::f64::consts::PI / 180.0).sin();
    let x0 = xy2[0].0 * cos_r - xy2[0].1 * sin_r;
    let x1 = xy2[1].0 * cos_r - xy2[1].1 * sin_r;
    let y0 = xy2[0].0 * sin_r + xy2[0].1 * cos_r;
    let par_move_x = -(x0 + x1) / 2.0;
    let par_move_y = -y0;
    // println!("par: {par_move_x} {par_move_y}");

    for ab in abn {
        let cos_r = (deg_max * std::f64::consts::PI / 180.0).cos();
        let sin_r = (deg_max * std::f64::consts::PI / 180.0).sin();
        let xx = ab.0 * cos_r - ab.1 * sin_r + par_move_x;
        let yy = ab.0 * sin_r + ab.1 * cos_r + par_move_y;
        println!("{} {}", xx, yy);
    }
}
