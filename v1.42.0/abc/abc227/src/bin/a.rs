use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize,
    }
    // 1-indexed で計算するなら (a + k - 1) % n を計算後 0 を n に置き換える
    // 0-indexed で計算するなら予め a の値をもうひとつ引いておき最後に 1-indexed に戻す
    println!("{}", (a + k - 2) % n + 1);
}
