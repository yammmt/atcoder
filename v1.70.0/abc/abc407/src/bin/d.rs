use proconio::fastout;
use proconio::input;

fn dfs(i: usize, j: usize, grid: &mut Vec<Vec<usize>>, ahw: &Vec<Vec<usize>>) -> usize {
    let h = grid.len();
    let w = grid[0].len();

    if i == h - 1 && j == w - 1 {
        // 最終マスには新たに置けない
        let mut score = 0;
        for ii in 0..h {
            for jj in 0..w {
                if grid[ii][jj] == 0 {
                    score ^= ahw[ii][jj];
                }
            }
        }
        return score;
    }

    let i_next = if j % w == w - 1 { i + 1 } else { i };
    let j_next = if j % w == w - 1 { 0 } else { j + 1 };

    // 置かない
    let s1 = dfs(i_next, j_next, grid, ahw);
    // 右に置く
    let s2 = if grid[i][j] == 0 && j != w - 1 && grid[i][j + 1] == 0 {
        grid[i][j] = 1;
        grid[i][j + 1] = 1;
        let s = dfs(i_next, j_next, grid, ahw);
        grid[i][j] = 0;
        grid[i][j + 1] = 0;
        s
    } else {
        0
    };
    // 下に置く
    // 下に既に置かれていることはないはず…
    let s3 = if grid[i][j] == 0 && i != h - 1 {
        grid[i][j] = 1;
        grid[i + 1][j] = 1;
        let s = dfs(i_next, j_next, grid, ahw);
        grid[i][j] = 0;
        grid[i + 1][j] = 0;
        s
    } else {
        0
    };

    s1.max(s2.max(s3))
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[usize; w]; h],
    }

    // hw <= 20 と小さい
    // 横長に置く, 縦長に置く, 置かない, で 3^20 通りを全列挙すると 3e9 通り超あって TLE
    // シンプルにドミノが置かれているか否かで 2^20 通り取って, 不正パターンを除外すればよい
    // 不正パターンの除外が難しい, 正パターンを DFS した方がよい
    // (a, b) を a*w + b にマッピングして一次元で管理する...より二次元で見た方がよさそう
    // いやでも都度 grid 全部渡すと計算重いのでダメそう, clone 避けて変更を戻しつつ

    let mut g = vec![vec![0; w]; h];
    println!("{}", dfs(0, 0, &mut g, &ahw));
}
