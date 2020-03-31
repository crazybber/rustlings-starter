// iterators4.rs
// Execute `rustlings hint iterators4` for hints.
// Do not use:
// - return //不要用return
// For extra fun don't use:
// - imperative style loops (for, while) //不要用循环
// - additional variables //不要用额外变量
// For the most fun don't use:
// - recursion //不要用递归

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return factorial of num
    match num {
        0 => 0,
        1 => 1,
        num => (1..num + 1).fold(1, |acc, x| x * acc),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
