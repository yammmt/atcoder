use proconio::input;

fn main() {
    input! {
        n: usize,
        apxn: [(i64, i64, i64); n],
    }

    let mut ans = std::i64::MAX / 2;
    for apx in &apxn {
        if apx.0 >= apx.2 {
            continue;
        }

        ans = ans.min(apx.1);
    }


    println!(
        "{}",
        if ans == std::i64::MAX / 2 {
            -1
        } else {
            ans
        }
    );
}
