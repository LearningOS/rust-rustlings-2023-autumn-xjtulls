// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.


// 这段代码是使用 Rust 编程语言编写的，它使用了 Rust 标准库中的 `std::iter::successors` 函数来创建一个迭代器，并且计算该迭代器的元素的乘积。让我来解释这段代码的每个部分：

// 1. `std::iter::successors(Some(1), |&n| { ... })`:
//    - `std::iter::successors` 是一个 Rust 标准库中的函数，它用于创建一个新的迭代器。
//    - `Some(1)` 是迭代器的起始点，表示从整数 1 开始迭代。
//    - `|&n| { ... }` 是一个闭包，它定义了迭代器中的每个元素是如何计算的。在这个闭包中，`n` 是当前的迭代元素。

// 2. 闭包 `|&n| { ... }` 的内容:
//    - 这个闭包接受一个整数 `n` 作为参数，并且返回一个 `Option` 枚举值。
//    - 在闭包内部，它首先检查 `n` 是否小于某个 `num`，其中 `num` 并没有在提供的代码中定义，这可能是一个变量的名字或者是在作用域之外定义的。
//    - 如果 `n` 小于 `num`，则返回 `Some(n + 1)`，表示下一个迭代元素是 `n` 增加 1。
//    - 否则，返回 `None`，表示迭代结束。

// 3. `.product()`:
//    - 这是迭代器上的方法，用于计算迭代器中元素的乘积。

// 总结：这段代码创建了一个从整数1开始的迭代器，每个元素是前一个元素加1，直到某个条件满足（`n` 不小于某个 `num`）。然后，它计算了迭代器中所有元素的乘积。这段代码的行为取决于定义的 `num` 变量，这个变量没有在提供的代码中显示定义，可能在代码的其他部分定义了。

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    std::iter::successors(Some(1),|&n|{
        if n < num {
            Some(n + 1)
        }else {
            None
        }
    }).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

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
