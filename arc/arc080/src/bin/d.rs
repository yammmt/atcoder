use proconio::input;

#[allow(clippy::collapsible_if)]
#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        an: [u32; n],
    }

    let mut mass = vec![vec![0; w]; h];
    let mut curn = 1;
    let mut curh = 0;
    let mut curw = 0;
    let mut go_down = true;
    for a in &an {
        for _ in 0..*a {
            mass[curh][curw] = curn;
            if go_down {
                if curh == h - 1 {
                    go_down = false;
                    curw += 1;
                } else {
                    curh += 1;
                }
            } else {
                if curh == 0 {
                    go_down = true;
                    curw += 1;
                } else {
                    curh -= 1;
                }
            }
        }
        curn += 1;
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", mass[i][j]);
            if j != w - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
