use proconio::input;

fn main() {
    input! {
        n: usize,
        xn: [f64; n],
    }
    println!("{}", xn.iter().map(|&x| x.abs()).sum::<f64>());
    println!("{}", xn.iter().map(|&x| x * x).sum::<f64>().sqrt());
    let mut cmax = 0.0f64;
    xn.iter().for_each(|&x| cmax = cmax.max(x.abs()));
    println!("{}", cmax);
}
