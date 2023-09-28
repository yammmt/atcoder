use proconio::input;

fn main() {
    input! {
        n: usize,
        xyhn: [(i64, i64, i64); n],
    }

    let mut nonzero = xyhn[0];
    for xyh in &xyhn {
        if xyh.2 != 0 {
            nonzero = *xyh;
            break;
        }
    }

    for cx in 0..101 {
        for cy in 0..101 {
            // println!("{}, {}", cx, cy);
            let mut passes = true;
            let mandiff = (cx - nonzero.0).abs() + (cy - nonzero.1).abs();
            let h = nonzero.2 + mandiff;
            // println!("{}", h);

            for xyh in &xyhn {
                // println!("tested: {:?}", xyh);
                let mandiff = (cx - xyh.0).abs() + (cy - xyh.1).abs();
                // println!("mandiff: {}", mandiff);
                if (h - mandiff <= 0 && xyh.2 != 0) || (h - mandiff > 0 && xyh.2 != h - mandiff) {
                    passes = false;
                    break;
                }
            }

            if passes {
                println!(
                    "{} {} {}",
                    cx,
                    cy,
                    h
                );
                return;
            }
        }
    }
}
