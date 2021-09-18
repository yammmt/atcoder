// :fu: :fu: 21-09 文法から何から 300 点にかける労力ではなかった
// 高校レベルの数問 重実装

use proconio::input;

// 転置して行順を逆にする
fn turn_left(sin: &Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    let mut ans = vec![vec![' '; n]; n];
    for i in 0..n {
        for j in 0..n {
            ans[i][j] = sin[j][n - i - 1];
        }
    }
    ans
}

// 外周を固定してオフセット踏まえて中身が一致するか？
fn check_same(sin: &Vec<Vec<char>>, tin: &Vec<Vec<char>>, n: usize) -> bool {
    let mut left_begin_s = n - 1;
    let mut up_begin_s = n - 1;
    let mut right_end_s = 0;
    let mut down_end_s = 0;
    for i in 0..n {
        for j in 0..n {
            if sin[i][j] == '#' {
                up_begin_s = up_begin_s.min(i);
                down_end_s = down_end_s.max(i);
                left_begin_s = left_begin_s.min(j);
                right_end_s = right_end_s.max(j);
            }
        }
    }
    // println!("{} {}", up_begin_s, left_begin_s);
    // println!("{} {}", down_end_s, right_end_s);

    let mut left_begin_t = n - 1;
    let mut up_begin_t = n - 1;
    let mut right_end_t = 0;
    let mut down_end_t = 0;
    for i in 0..n {
        for j in 0..n {
            if tin[i][j] == '#' {
                up_begin_t = up_begin_t.min(i);
                down_end_t = down_end_t.max(i);
                left_begin_t = left_begin_t.min(j);
                right_end_t = right_end_t.max(j);
            }
        }
    }
    // println!("{} {}", up_begin_t, left_begin_t);
    // println!("{} {}", down_end_t, right_end_t);

    if right_end_s - left_begin_s != right_end_t - left_begin_t
        || down_end_s - up_begin_s != down_end_t - up_begin_t
    {
        // println!("size mismatch");
        return false;
    }

    let i_len = down_end_s - up_begin_s + 1;
    let j_len = right_end_t - left_begin_t + 1;
    for i in 0..i_len {
        let si = up_begin_s + i;
        let ti = up_begin_t + i;
        for j in 0..j_len {
            let sj = left_begin_s + j;
            let tj = left_begin_t + j;
            if sin[si][sj] != tin[ti][tj] {
                return false;
            }
        }
    }

    true
}

fn main() {
    input! {
        n: usize,
        _snn: [String; n],
        _tnn: [String; n],
    }
    let mut snn = vec![];
    for s in &_snn {
        let mut row = vec![];
        for c in s.chars() {
            row.push(c);
        }
        snn.push(row);
    }
    // println!("{:?}", snn);
    let mut tnn = vec![];
    for t in &_tnn {
        let mut row = vec![];
        for c in t.chars() {
            row.push(c);
        }
        tnn.push(row);
    }

    // n <= 200 だし全探索？
    // 平行移動は無視して 90 度回転させた 3 通りのマスについて左上を固定するなど？
    for _ in 0..4 {
        // for s in &snn {
        //     println!("{:?}", s);
        // }

        if check_same(&snn, &tnn, n) {
            println!("Yes");
            return;
        }

        snn = turn_left(&snn, n);
    }

    println!("No");
}
