use proconio::input;

// n^p mod m (繰り返し二乗法)
fn repeat_square(n: u64, p: u64, m: u64) -> u64 {
    if p == 0 {
        1
    } else if p == 1 {
        n % m
    } else if p % 2 == 0 {
        repeat_square(n, p / 2, m).pow(2) % m
    } else {
        (n * repeat_square(n, p - 1, m)) % m
    }
}

fn main() {
    input! {
        n: usize,
        dn: [u64; n],
    }
    let d = 998244353;

    if dn[0] != 0 {
        // 頂点 1 と頂点 1 の距離は 0 でなくてはならない
        println!("0");
        return;
    }

    let mut vd = vec![0; n];
    for dd in &dn {
        if *dd > n as u64 - 1 {
            // どうしても到達不可
            println!("0");
            return;
        }

        vd[*dd as usize] += 1;
    }
    // println!("{:?}", vd);

    let mut ans = 1;
    let mut appeared = 1;
    for i in 1..n {
        if i != 1 && vd[i - 1] == 0 {
            // 距離 i に達するには距離 i - 1 の頂点が必須
            println!("0");
            return;
        }

        ans = (ans * repeat_square(vd[i - 1], vd[i], d)) % d;
        appeared += vd[i];
        if appeared == n as u64 {
            break;
        }
    }
    println!("{}", ans);
}
