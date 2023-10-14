use proconio::input;

fn main() {
    input! {
        r: i64,
        x: i64,
        y: i64,
    }

    // 通り過ぎる
    if r * r > x * x + y * y {
        println!("2");
        return;
    }

    let bunshi = x * x + y * y;
    let bunbo = r * r;
    let required = (bunshi + bunbo - 1) / bunbo;
    // println!("req: {required}");

    let mut i = 0;
    loop {
        if i * i >= required {
            println!("{i}");
            return;
        }

        i += 1;
    }
}
