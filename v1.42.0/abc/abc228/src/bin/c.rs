// :fu: 21-11 算数嫌

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        pn3: [[usize; 3]; n],
    }

    let mut pn = vec![];
    for pnn in &pn3 {
        pn.push(pnn.iter().sum::<usize>());
    }
    pn.sort_unstable();
    pn.reverse();

    for pnn in &pn3 {
        let max_score = pnn.iter().sum::<usize>() + 300;
        println!("{}", if max_score >= pn[k - 1] { "Yes" } else { "No" });
    }

    // :fu: :fu: :fu: 二分探索はなぜか死ぬ
    // for pnn in &pn3 {
    //     let max_score = pnn.iter().sum::<usize>() + 300;
    //     // 自身よりスコアが良い人が upper 人
    //     let mut upper = 0;
    //     let mut lower = n as isize + 1;
    //     while lower - upper > 1 {
    //         let mid = (upper + lower) / 2;
    //         // println!("{}", max_score);
    //         if max_score > pn[mid as usize] {
    //             lower = mid;
    //         } else {
    //             upper = mid;
    //         }
    //         // println!(" mid: {}", mid);
    //         // println!(" {} {}", upper, lower);
    //     }
    //     println!("  {} {}", upper, lower);
    //     // かつての自身の順位は確実に満点後の自身より下に位置する
    //     println!(
    //         "{}",
    //         if upper < k as isize {
    //             "Yes"
    //         } else {
    //             "No"
    //         }
    //     );
    // }
}
