use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        an: [Usize1; n],
    }
    let mut has_item_count = 0;
    let mut has_item = vec![false; m];
    for (i, &a) in an.iter().enumerate() {
        if has_item[a] {
            continue;
        }

        has_item[a] = true;
        has_item_count += 1;
        if has_item_count == m {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
