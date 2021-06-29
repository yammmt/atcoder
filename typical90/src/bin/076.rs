// :fu: :fu: 21-06 体感難易度あってないので苦手分野 しゃくとり

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [u64; n],
    }
    let ansum = an.iter().sum::<u64>();

    // そもそも 1/10 となりえない
    if ansum % 10 != 0 {
        println!("No");
        return;
    }

    for i in 0..n {
        an.push(an[i]);
    }
    let an = an;

    let mut cusum = 0;
    let mut right = 0;
    for left in 0..n {
        while right < 2 * n && cusum < ansum / 10 {
            cusum += an[right];
            right += 1;
        }

        if cusum == ansum / 10 {
            println!("Yes");
            return;
        }

        // 左右一致判定は省いても通る
        cusum -= an[left];
    }

    println!("No");
}
