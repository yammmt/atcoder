use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            a: u64,
            s: u64,
        }

        // x AND y = a より a で bit 1 が立っている箇所は x/y 共にすべて bit 1 固定
        // x, y の最小値が a と同値になるのでとりあえず x = y = a とする
        // 後は s - 2*a の値を x, y の片方に足してやれば x + y = s を満たすようになる
        // ここで, 先の式に戻り x AND y != a となれば No
        if 2 * a > s {
            // x, y の最小値が a と同値 => x + y >= 2a
            println!("No");
            continue;
        }

        println!(
            "{}",
            if (s - 2 * a) & a == 0 {
                "Yes"
            } else {
                "No"
            }
        );
    }
}
