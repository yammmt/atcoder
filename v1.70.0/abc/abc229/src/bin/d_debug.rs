fn wa(s: &Vec<char>, k: usize) -> usize {
    let n = s.len();

    let mut ans = 0;
    let mut right = 0;
    let mut dot_count = 0;
    for left in 0..n {
        if right < left {
            right = left;
        }
        while right < n && (s[right] == 'X' || dot_count < k) {
            if s[right] == '.' {
                dot_count += 1;
            }
            right += 1;
        }

        // println!("  {left}, {right}");
        ans = ans.max(right - left);
        if s[left] == '.' && dot_count > 0 {
            dot_count -= 1;
        }
    }
    ans
}

fn ac(s: &Vec<char>, k: usize) -> usize {
    let n = s.len();

    let mut ans = 0;
    let mut right = 0;
    let mut dot_count = 0;
    for left in 0..n {
        while right < n {
            if s[right] == '.' {
                if dot_count == k {
                    break;
                }
                dot_count += 1;
            }
            right += 1;
        }

        ans = ans.max(right - left);
        if s[left] == '.' {
            dot_count -= 1;
        }
    }
    ans
}

fn main() {
    for bit in 1..3usize.pow(10u32) {
        let mut s = vec![];
        let mut b = bit;
        while b > 0 {
            match b % 3 {
                1 => s.push('X'),
                2 => s.push('.'),
                _ => {}
            }
            b /= 3;
        }

        for i in 0..13 {
            println!("{:?}, {i}", s);
            assert_eq!(ac(&s, i), wa(&s, i));
        }
    }
}
