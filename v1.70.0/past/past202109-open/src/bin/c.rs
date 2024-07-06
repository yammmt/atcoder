use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        an: [usize; n],
    }

    println!("{}", an.iter().filter(|&&a| a == x).count());
}
