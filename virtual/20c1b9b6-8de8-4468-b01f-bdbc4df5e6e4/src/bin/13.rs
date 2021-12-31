use proconio::input;

fn to_decimal(mut n: usize, k: usize) -> usize {
    let mut vn = vec![];
    while n > 0 {
        vn.push(n % 10);
        n /= 10;
    }

    let mut ret = 0;
    let mut cur = 1;
    for nn in vn {
        ret += cur * nn;
        cur *= k;
    }
    ret
}

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize,
    }
    println!("{}", to_decimal(a, k) * to_decimal(b, k));
}
