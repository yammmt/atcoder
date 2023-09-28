// WA: [i] を [0] としてしまっていたら中途半端に WA が出てデバッグ死亡 (23AC 5WA) (+30min 程)

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        vpn: [(u64, u64); n],
    }

    let mut curx = 0;
    for (i, vp) in vpn.iter().enumerate() {
        curx += vp.0 * vp.1;
        if curx > 100 * x {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
