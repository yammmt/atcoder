use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        let mut x = an[i];
        let mut cur = 1;
        while x - 1 != i {
            x = an[x - 1];
            cur += 1;
        }
        ans.push(cur);
    }

    for (i, a) in ans.iter().enumerate() {
        print!("{}", a);

        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
