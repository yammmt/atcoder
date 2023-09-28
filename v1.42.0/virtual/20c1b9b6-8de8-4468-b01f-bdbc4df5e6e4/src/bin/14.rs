use proconio::input;

fn main() {
    input! {
        n: usize,
        tn: [i64; n],
        m: usize,
        pxm: [(usize, i64); m],
    }

    let default_ans = tn.iter().sum::<i64>();
    for px in &pxm {
        println!("{}", default_ans - tn[px.0 - 1] + px.1);
    }
}
