use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [u64; n],
    }

    println!("{}", an.iter().sum::<u64>() - n as u64);
}
