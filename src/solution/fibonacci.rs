// Create a program that prints the Fibonacci sequence up to n terms.

use std::mem;

fn fib(n: i32) -> Vec<i32> {
    let mut fib: Vec<i32> = vec![0, 1];
    let mut n1 = 0_i32;
    let mut n2 = 1_i32;

    for _ in 2..n {
        mem::swap(&mut n1, &mut n2);
        n2 = n1 + n2;
        fib.push(n2);
    }

    fib
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(vec! {0_i32,1_i32,1_i32,2_i32,3_i32,5_i32}, fib(6));
    }
}
