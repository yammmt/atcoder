use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut prime_num = vec![0; n + 1];
    for i in 2..n + 1 {
        if prime_num[i] != 0 {
            continue;
        }

        let mut j = i;
        while j < n + 1 {
            prime_num[j] += 1;
            j += i;
        }
    }

    println!("{}", prime_num.iter().filter(|&a| *a >= k).count());
}
