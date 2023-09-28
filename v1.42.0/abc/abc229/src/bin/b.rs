use proconio::input;

fn to_digit_array(mut n: usize) -> Vec<usize> {
    let mut ret = vec![];
    while n > 0 {
        ret.push(n % 10);
        n /= 10;
    }
    ret.reverse();
    ret
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut va = to_digit_array(a);
    let mut vb = to_digit_array(b);

    if va.len() < vb.len() {
        va.reverse();
        while va.len() < vb.len() {
            va.push(0);
        }
        va.reverse();
    }
    if vb.len() < va.len() {
        vb.reverse();
        while vb.len() < va.len() {
            vb.push(0);
        }
        vb.reverse();
    }

    let c = a + b;
    let vc = to_digit_array(c);

    if vc.len() != va.len() {
        println!("Hard");
        return;
    }

    for i in 0..vc.len() {
        if va[i] + vb[i] != vc[i] {
            println!("Hard");
            return;
        }
    }

    println!("Easy");
}
