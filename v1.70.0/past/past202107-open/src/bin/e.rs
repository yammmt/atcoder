use proconio::input;

fn main() {
    input! {
        n: u128,
    }

    // i 回目の操作後に加算
    for i in 1..=30 {
        let mut x = 1u128;
        for j in 1..=30 {
            x *= 3;
            if i == j {
                x += 1;
            }
        }

        if x == n {
            println!("{i}");
            return;
        }
    }

    println!("-1");
}
