use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n],
    }

    // 先手が勝つなら true
    let mut dp = vec![false; k + 1];
    for i in 0..k + 1 {
        if dp[i] {
            continue;
        }

        for a in &an {
            let next_i = i + *a;
            if next_i > k {
                break;
            }

            dp[next_i] = true;
        }
    }
    // println!("{:?}", dp);

    println!(
        "{}",
        if dp[k] {
            "First"
        } else {
            "Second"
        }
    );
}
