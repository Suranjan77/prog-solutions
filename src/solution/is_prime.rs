//Implement a function that checks if a number is prime.

fn is_prime(num: i32) -> bool {
    match num {
        2 | 3 => true,
        n => {
            for r in 2..n {
                if num % r == 0 {
                    return false;
                }
            }

            return true;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_prime() {
        assert_eq!(true, is_prime(19));
    }

    #[test]
    fn test_non_prime() {
        assert_eq!(false, is_prime(8));
    }
}
