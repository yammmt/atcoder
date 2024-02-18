use proconio::fastout;
use proconio::input;

fn is_triangle(a: usize, b: usize, c: usize) -> bool {
    let ab = a + b;
    let bc = b + c;
    let ca = c + a;
    ab > c && bc > a && ca > b
}

fn dfs(an: &Vec<usize>) -> usize {
    let n = an.len();
    if n == 0 {
        return 1;
    }

    let mut ret = 0;
    for i in 1..n {
        for j in i + 1..n {
            if is_triangle(an[0], an[i], an[j]) {
                let mut v = Vec::with_capacity(n - 3);
                for (k, a) in an.iter().enumerate() {
                    if k == 0 || k == i || k == j {
                        continue;
                    }

                    v.push(*a);
                }
                ret += dfs(&v);
            }
        }
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a3n: [usize; 3 * n]
    }

    println!("{}", dfs(&a3n));
}
