use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut pn: [usize; n],
    }
    pn.sort_unstable();
    println!("{}", pn.iter().take(k).sum::<usize>());
}
