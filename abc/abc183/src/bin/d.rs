use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i64,
        stpn: [(usize, usize, i64); n],
    }
    // println!("stpn: {:?}", stpn);

    let mut tmax = 0;
    for i in &stpn {
        tmax = tmax.max(i.1);
    }
    // println!("tmax: {}", tmax);

    let mut imos = vec![0i64; tmax + 1];
    for i in &stpn {
        imos[i.0] += i.2 as i64;
        imos[i.1] -= i.2 as i64;
    }
    // println!("{:?}", imos);

    let mut curout = 0;
    for i in &imos {
        curout += *i;
        // println!("{}", curout);
        if curout > w {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
