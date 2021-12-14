use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [u64; n],
    }

    // 先頭を一度, 以後若い順に二度ずつ値を足していく (貪欲)
    an.sort_unstable();
    an.reverse();
    let mut ans = 0;
    (0..n - 1).for_each(|i| ans += an[(i + 1) / 2]);

    println!("{}", ans);
}
