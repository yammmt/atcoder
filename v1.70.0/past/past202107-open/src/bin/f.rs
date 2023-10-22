use proconio::input;

fn main() {
    input! {
        n: usize,
        mut dstn: [(usize, usize, usize); n],
    }

    // 終了時間昇順
    dstn.sort_unstable_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else if a.2 != b.2 {
            a.2.cmp(&b.2)
        } else {
            a.1.cmp(&b.1)
        }
    });
    // println!("{:?}", dstn);

    for i in 1..n {
        if dstn[i].0 == dstn[i - 1].0 && dstn[i].1 < dstn[i - 1].2 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
