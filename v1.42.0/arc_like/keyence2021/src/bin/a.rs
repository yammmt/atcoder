use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
        bn: [i64; n],
    }

    let mut cn = Vec::with_capacity(n);
    let mut amax = 0;
    let mut abmax = 0;
    // それまでの最大 or a 最大と新しい b
    for i in 0..n {
        amax = amax.max(an[i]);
        let with_a = amax * bn[i];
        cn.push(with_a.max(abmax));
        abmax = cn[i];
    }

    for c in &cn {
        println!("{}", c);
    }
}
