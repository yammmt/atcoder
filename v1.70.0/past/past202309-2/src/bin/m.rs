// https://qiita.com/zu_rin/items/09876d2c7ec12974bc0f

use proconio::fastout;
use proconio::input;

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    // 向きベクトルを計算
    fn direction(&self, p: &Point) -> f64 {
        (self.end.x - self.start.x) * (p.y - self.start.y)
            - (self.end.y - self.start.y) * (p.x - self.start.x)
    }

    // 2つの線分が交差しているかどうかを判定
    fn intersects(&self, other: &Line) -> bool {
        let direction1 = self.direction(&other.start);
        let direction2 = self.direction(&other.end);
        let direction3 = other.direction(&self.start);
        let direction4 = other.direction(&self.end);

        if direction1 == 0.0 && direction2 == 0.0 && direction3 == 0.0 && direction4 == 0.0 {
            return self.on_segment(&other.start)
                || self.on_segment(&other.end)
                || other.on_segment(&self.start)
                || other.on_segment(&self.end);
        }

        (direction1 * direction2 <= 0.0) && (direction3 * direction4 <= 0.0)
    }

    fn on_segment(&self, p: &Point) -> bool {
        p.x >= f64::min(self.start.x, self.end.x)
            && p.x <= f64::max(self.start.x, self.end.x)
            && p.y >= f64::min(self.start.y, self.end.y)
            && p.y <= f64::max(self.start.y, self.end.y)
    }
}

#[fastout]
fn main() {
    input! {
        // x y x y の順
        abcd0: (f64, f64, f64, f64),
        abcd1: (f64, f64, f64, f64),
    }

    // 1WA はどこかの例外？
    // 境界判定も typo もないと思うが
    // 値の範囲も問題ないはず

    // 最初の WA: これが Y
    // 1 1 2 2
    // 3 3 4 4
    // 直線の一致判定を取ってしまうので

    let ax = abcd0.0;
    let ay = abcd0.1;
    let bx = abcd0.2;
    let by = abcd0.3;

    let cx = abcd1.0;
    let cy = abcd1.1;
    let dx = abcd1.2;
    let dy = abcd1.3;

    let line1 = Line {
        start: Point { x: ax, y: ay },
        end: Point { x: bx, y: by },
    };

    let line2 = Line {
        start: Point { x: cx, y: cy },
        end: Point { x: dx, y: dy },
    };

    if line1.intersects(&line2) {
        println!("Yes");
    } else {
        println!("No");
    }

    // // FIXME: 平行時には座標が被るか否かでみなくてはならないっぽい
    // // 平行というか四点が一直線上でよい？
    // // でもこれを足すと 2WA に増える
    // // 0 0 0 10
    // // 0 12 0 15
    // let cl = (cx - ax) * (by - ay);
    // let cr = (cy - ay) * (bx - ax);
    // let dl = (dx - ax) * (by - ay);
    // let dr = (dy - ay) * (bx - ax);
    // if cl == cr && dl == dr {
    //     // 四点は一直線上
    //     let ab_x_min = ax.min(bx);
    //     let ab_x_max = ax.max(bx);
    //     let cd_x_min = cx.min(dx);
    //     let cd_x_max = cx.max(dx);
    //     println!(
    //         "{}",
    //         if (ab_x_min..=ab_x_max).contains(&cd_x_min)
    //             || (ab_x_min..=ab_x_max).contains(&cd_x_max)
    //         {
    //             "Yes"
    //         } else {
    //             "No"
    //         }
    //     );
    //     return;
    // }

    // let mut s_top = (cx - ax) * (dy - cy) - (cy - ay) * (dx - cx);
    // let mut s_btm = (bx - ax) * (dy - cy) - (by - ay) * (dx - cx);
    // let mut t_top = (ax - cx) * (by - ay) - (ay - cy) * (bx - ax);
    // let mut t_btm = (dx - cx) * (by - ay) - (dy - cy) * (bx - ax);

    // // println!("{s_top}/{s_btm}");
    // // println!("{t_top}/{t_btm}");
    // if s_top <= 0 && s_btm <= 0 {
    //     s_top *= -1;
    //     s_btm *= -1;
    // }
    // if t_top <= 0 && t_btm <= 0 {
    //     t_top *= -1;
    //     t_btm *= -1;
    // }

    // let s_pass = 0 <= s_top && s_top <= s_btm;
    // let t_pass = 0 <= t_top && t_top <= t_btm;
    // // println!("{s_top}/{s_btm}");
    // // println!("{t_top}/{t_btm}");

    // println!("{}", if s_pass && t_pass { "Yes" } else { "No" });
}
