use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n - 1],
    }

    let mut ans = vec![0; n + 1];
    an.iter().for_each(|&a| ans[a] += 1);

    for a in ans.iter().skip(1) {
        println!("{}", a);
    }
}
