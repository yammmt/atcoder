use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
        bn: [usize; n],
    }

    let ans = (0..n).filter(|&i| an[i] == bn[i]).count();
    println!("{ans}");

    let mut ans = 0;
    for (i, &a) in an.iter().enumerate() {
        ans += (0..n).filter(|&j| a == bn[j] && i != j).count();
    }
    println!("{ans}");
}
