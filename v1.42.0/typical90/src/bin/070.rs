use proconio::input;

fn main() {
    input! {
        n: usize,
        xyn: [(i64, i64); n],
    }

    let mut xn = xyn.iter().map(|xy| xy.0).collect::<Vec<i64>>();
    let mut yn = xyn.iter().map(|xy| xy.1).collect::<Vec<i64>>();
    xn.sort();
    yn.sort();
    let xmid = if n % 2 == 0 {
        (xn[n / 2] + xn[n / 2 - 1]) / 2
    } else {
        xn[n / 2]
    };
    let ymid = if n % 2 == 0 {
        (yn[n / 2] + yn[n / 2 - 1]) / 2
    } else {
        yn[n / 2]
    };

    let mut ans = 0;
    xyn.iter().for_each(|xy| {
        ans += (xy.0 - xmid).abs() + (xy.1 - ymid).abs();
    });

    println!("{}", ans);
}
