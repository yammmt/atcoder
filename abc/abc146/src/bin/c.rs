// 16min 1WA

use proconio::input;

fn dn(mut n: u64) -> u64 {
    let mut ret = 0;
    while n > 0 {
        ret += 1;
        n /= 10;
    }
    ret
}

fn main() {
    input! {
        a: u64,
        b: u64,
        x: u64,
    }

    let mut kaeru = 0;
    // 所持金 x を初期値とすると a * x が最大 10^27 となり u64 範囲外
    // 10^9 で一回判定取ってやった方が良いが, 制約により買えない値を買えないと言い張ってやった方が簡潔
    let mut kaenai = 10u64.pow(9) + 1;
    while kaenai - kaeru > 1 {
        if kaeru >= 10u64.pow(9) {
            println!("{}", 10u64.pow(9));
            return;
        }

        let cur = (kaeru + kaenai) / 2;
        if a * cur + b * dn(cur) <= x {
            kaeru = cur;
        } else {
            kaenai = cur;
        }
    }
    println!("{}", kaeru);
}
