use num::Num;

pub fn abs_diff<N: Num + PartialOrd>(a: N, b: N) -> N {
    if a > b {
        a - b
    } else {
        b - a
    }
}
