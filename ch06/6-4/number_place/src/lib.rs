use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const N: usize = 81;

fn is_valid(result: [u8; N], p: usize, v: u8) -> bool {
    let y = p / 9;
    let x = p % 9;
    for i in 0..9 {
        if result[9 * i + x] == v || result[9 * y + i] == v {
            return false;
        }
    }
    let block_y = y / 3;
    let block_x = x / 3;
    for i in 0..3 {
        for j in 0..3 {
            if result[9 * (3 * block_y + i) + (3 * block_x + j)] == v {
                return false;
            }
        }
    }
    return true;
}

#[wasm_bindgen]
pub fn solve(problem: Vec<u8>) -> Vec<u8> {
    solve_innner(problem)
}

fn solve_innner(problem: Vec<u8>) -> Vec<u8> {
    let mut result = [0; N];

    let mut stack = vec![];
    for i in 0..N {
        if problem[i] > 0 {
            result[i] = problem[i];
        } else if stack.is_empty() {
            stack.push((false, i, 1));
        }
    }

    let mut is_failing = false;
    while !stack.is_empty() {
        let (is_back, p, v) = stack.pop().unwrap();
        // 戻りがけの処理
        if is_back && is_failing {
            result[p] = 0;
            if v < 9 {
                stack.push((false, p, v + 1));
            }
            continue;
        }
        // 行きがけの処理
        if !is_valid(result, p, v) {
            if v < 9 {
                stack.push((false, p, v + 1));
            } else {
                is_failing = true;
            }
            continue;
        }
        is_failing = false;
        result[p] = v;
        stack.push((true, p, v));
        let mut is_updated = false;
        for i in p + 1..N {
            if result[i] == 0 {
                stack.push((false, i, 1));
                is_updated = true;
                break;
            }
        }
        if !is_updated {
            break;
        }
    }
    result.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_innner() {
        let p = vec![
            0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 0, 0, 1, 0, 9, 0, 0, 0, 0, 5, 0, 0, 0, 2, 0,
            0, 3, 1, 0, 0, 0, 0, 1, 4, 0, 6, 7, 8, 0, 0, 0, 7, 8, 6, 5, 2, 9, 0, 3, 4, 5, 6, 7, 0,
            9, 3, 0, 0, 1, 8, 9, 1, 4, 5, 6, 3, 7, 2, 0, 2, 4, 0, 8, 7, 5, 0, 9,
        ];
        let expected = vec![
            4, 7, 2, 9, 3, 1, 6, 5, 8, 6, 3, 5, 8, 4, 2, 9, 1, 7, 9, 1, 8, 7, 6, 5, 4, 2, 3, 2, 5,
            9, 3, 1, 4, 7, 8, 6, 1, 4, 3, 6, 7, 8, 2, 9, 5, 7, 8, 6, 5, 2, 9, 1, 3, 4, 5, 6, 7, 2,
            9, 3, 8, 4, 1, 8, 9, 1, 4, 5, 6, 3, 7, 2, 3, 2, 4, 1, 8, 7, 5, 6, 9,
        ];
        assert_eq!(solve_innner(p), expected);

        let p = vec![
            1, 0, 0, 0, 0, 8, 0, 4, 0, 0, 0, 8, 0, 0, 9, 0, 6, 0, 0, 4, 5, 0, 0, 2, 8, 0, 0, 8, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 7, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 7, 5,
            0, 0, 6, 2, 0, 0, 5, 0, 6, 0, 0, 7, 0, 0, 0, 2, 0, 3, 0, 0, 0, 0, 9,
        ];
        let expected = vec![
            1, 6, 2, 7, 3, 8, 9, 4, 5, 7, 3, 8, 4, 5, 9, 1, 6, 2, 9, 4, 5, 1, 6, 2, 8, 7, 3, 8, 7,
            3, 9, 4, 5, 2, 1, 6, 2, 1, 6, 8, 7, 3, 5, 9, 4, 5, 9, 4, 2, 1, 6, 3, 8, 7, 3, 8, 7, 5,
            9, 4, 6, 2, 1, 4, 5, 9, 6, 2, 1, 7, 3, 8, 6, 2, 1, 3, 8, 7, 4, 5, 9,
        ];
        assert_eq!(solve_innner(p), expected);

        let p = vec![
            0, 8, 0, 0, 0, 6, 5, 0, 0, 0, 0, 9, 0, 0, 7, 0, 0, 3, 5, 0, 0, 3, 0, 2, 0, 6, 0, 0, 4,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 7, 0, 8,
            0, 3, 0, 0, 4, 6, 0, 0, 1, 0, 0, 2, 0, 0, 0, 0, 3, 4, 0, 0, 0, 7, 0,
        ];
        let expected = vec![
            3, 8, 2, 9, 4, 6, 5, 1, 7, 4, 6, 9, 5, 1, 7, 8, 2, 3, 5, 1, 7, 3, 8, 2, 4, 6, 9, 9, 4,
            6, 7, 5, 1, 3, 8, 2, 7, 5, 1, 2, 3, 8, 9, 4, 6, 2, 3, 8, 6, 9, 4, 7, 5, 1, 1, 7, 5, 8,
            2, 3, 6, 9, 4, 6, 9, 4, 1, 7, 5, 2, 3, 8, 8, 2, 3, 4, 6, 9, 1, 7, 5,
        ];
        assert_eq!(solve_innner(p), expected);
    }

    #[test]
    fn test_is_valid() {
        let result = [0; N];
        for v in 1..=9 {
            assert_eq!(is_valid(result, 0, v), true);
        }
        let mut result = [0; N];
        for i in 1..=9 {
            result[i - 1] = i as u8;
        }
        assert_eq!(is_valid(result, 9, 1), false);
        assert_eq!(is_valid(result, 9, 2), false);
        assert_eq!(is_valid(result, 9, 3), false);
        for v in 4..=9 {
            assert_eq!(is_valid(result, 9, v), true);
        }
        assert_eq!(is_valid(result, 27, 1), false);
        assert_eq!(is_valid(result, 27, 2), true);
        assert_eq!(is_valid(result, 27, 3), true);
        for v in 4..=9 {
            assert_eq!(is_valid(result, 27, v), true);
        }
        result[35] = 1;
        assert_eq!(is_valid(result, 52, 1), false);
        for v in 2..=7 {
            assert_eq!(is_valid(result, 52, v), true);
        }
        assert_eq!(is_valid(result, 52, 8), false);
        assert_eq!(is_valid(result, 52, 9), true);
    }
}
