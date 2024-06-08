use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    // a が指数であるがために愚直操作が通りそうな気がする, O(N^2) もかからない？
    // => そもそもボールの削除は最大でも N-1 回

    let mut row = vec![];
    for a in an {
        row.push(a);
        while row.len() >= 2 {
            let l0 = row.pop().unwrap();
            let l1 = row.pop().unwrap();
            if l0 == l1 {
                row.push(l0 + 1);
            } else {
                row.push(l1);
                row.push(l0);
                break;
            }
        }
    }

    println!("{}", row.len());
}
