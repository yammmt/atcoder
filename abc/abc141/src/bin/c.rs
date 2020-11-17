use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        q: i64,
        an: [usize; q],
    }

    let mut pts = vec![0; n];
    for a in &an {
        pts[a - 1] += 1;
    }

    for i in &pts {
        if k - (q - i) <= 0 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
