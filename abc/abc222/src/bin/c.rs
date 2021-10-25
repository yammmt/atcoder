use proconio::input;
use proconio::marker::Chars;

fn a_win(a: char, b: char) -> bool {
    if (a == 'G' && b == 'C') || (a == 'C' && b == 'P') || (a == 'P' && b == 'G') {
        return true;
    }
    false
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a2nm: [Chars; 2 * n],
    }

    let mut wins = vec![0; 2 * n];
    for i in 0..m {
        let mut order = vec![];
        // (勝数, 番号)
        for j in 0..2 * n {
            order.push((m - wins[j], j));
        }
        order.sort_unstable();
        // println!("{:?}", order);

        for j in 0..n {
            // println!("j: {}", j);
            if a_win(a2nm[order[2 * j].1][i], a2nm[order[2 * j + 1].1][i]) {
                // println!("  {} += 1", order[2 * j].1);
                wins[order[2 * j].1] += 1;
            } else if a_win(a2nm[order[2 * j + 1].1][i], a2nm[order[2 * j].1][i]) {
                // println!("  {} += 1", order[2 * j + 1].1);
                wins[order[2 * j + 1].1] += 1;
            }
        }
    }
    // println!("{:?}", wins);

    let mut ans = vec![];
    // (勝数, 番号)
    for j in 0..2 * n {
        ans.push((m - wins[j], j + 1));
    }
    ans.sort_unstable();
    for a in &ans {
        println!("{}", a.1);
    }
}
