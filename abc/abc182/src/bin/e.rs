use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        abn: [(usize, usize); n],
        cdm: [(usize, usize); m],
    }

    let mut mass = vec![vec!['F'; w + 1]; h + 1];
    for ab in &abn {
        mass[ab.0][ab.1] = 'L';
    }
    for cd in &cdm {
        mass[cd.0][cd.1] = 'B';
    }

    // W 正方向
    for i in 1..h + 1 {
        let mut is_prev_lamp = false;
        for j in 1..w + 1 {
            if mass[i][j] == 'B' {
                is_prev_lamp = false;
            } else {
                if mass[i][j] == 'F' {
                    if is_prev_lamp {
                        mass[i][j] = 'T';
                    }
                } else if mass[i][j] == 'L' {
                    is_prev_lamp = true;
                }
            }
        }
    }
    // W 負方向
    for i in 1..h + 1 {
        let mut is_prev_lamp = false;
        for j in (1..w + 1).rev() {
            if mass[i][j] == 'B' {
                is_prev_lamp = false;
            } else {
                if mass[i][j] == 'F' {
                    if is_prev_lamp {
                        mass[i][j] = 'T';
                    }
                } else if mass[i][j] == 'L' {
                    is_prev_lamp = true;
                }
            }
        }
    }
    // H 正方向
    for j in 1..w + 1 {
        let mut is_prev_lamp = false;
        for i in 1..h + 1 {
            if mass[i][j] == 'B' {
                is_prev_lamp = false;
            } else {
                if mass[i][j] == 'F' {
                    if is_prev_lamp {
                        mass[i][j] = 'T';
                    }
                } else if mass[i][j] == 'L' {
                    is_prev_lamp = true;
                }
            }
        }
    }
    // H 負方向
    for j in 1..w + 1 {
        let mut is_prev_lamp = false;
        for i in (1..h + 1).rev() {
            if mass[i][j] == 'B' {
                is_prev_lamp = false;
            } else {
                if mass[i][j] == 'F' {
                    if is_prev_lamp {
                        mass[i][j] = 'T';
                    }
                } else if mass[i][j] == 'L' {
                    is_prev_lamp = true;
                }
            }
        }
    }
    // println!("{:?}", mass);


    // count 'L' and 'T'
    let mut ans = 0;
    for i in 1..h + 1 {
        for j in 1..w + 1 {
            if mass[i][j] == 'T' || mass[i][j] == 'L' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
