use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let mut cnt = vec![0; 11];
    cnt[a] += 1;
    cnt[b] += 1;
    cnt[c] += 1;
    println!(
        "{}",
        if cnt[5] == 2 && cnt[7] == 1 {
            "YES"
        } else {
            "NO"
        }
    );
}
