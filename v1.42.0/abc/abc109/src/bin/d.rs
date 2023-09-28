use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[usize; w]; h],
    }

    // 一筆書きして奇数コインを運ぶ (最短手数である必要はない)
    let mut ans = vec![];
    let mut reverse_order = false;
    let mut moved_from = None;
    for i in 0..h {
        if reverse_order {
            for j in (0..w).rev() {
                if moved_from.is_none() {
                    if ahw[i][j] % 2 == 1 {
                        moved_from = Some((i, j));
                    }
                } else {
                    let prev = moved_from.unwrap();
                    ans.push((prev.0 + 1, prev.1 + 1, i + 1, j + 1));
                    moved_from = if ahw[i][j] % 2 == 1 {
                        None
                    } else {
                        Some((i, j))
                    };
                }
            }
        } else {
            for j in 0..w {
                if moved_from.is_none() {
                    if ahw[i][j] % 2 == 1 {
                        moved_from = Some((i, j));
                    }
                } else {
                    let prev = moved_from.unwrap();
                    ans.push((prev.0 + 1, prev.1 + 1, i + 1, j + 1));
                    moved_from = if ahw[i][j] % 2 == 1 {
                        None
                    } else {
                        Some((i, j))
                    };
                }
            }
        }
        reverse_order = !reverse_order;
    }

    println!("{}", ans.len());
    for a in &ans {
        println!("{} {} {} {}", a.0, a.1, a.2, a.3);
    }
}
