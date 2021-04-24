use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [i64; n],
    }

    let mut cur = an.iter().take(k).sum::<i64>();
    for i in 0..n {
        println!("{}", cur);
        cur -= an[i];
        let next_i = i + k;
        if next_i >= n {
            break;
        }

        cur += an[next_i];
    }
}
