use proconio::input;

fn main() {
    input! {
        n: usize,
        pn: [usize; n],
    }

    let mut ans = vec![0; n];
    for (i, p) in pn.iter().enumerate() {
        ans[*p - 1] = i + 1;
    }

    for (i, a) in ans.iter().enumerate() {
        print!("{}", a);
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
