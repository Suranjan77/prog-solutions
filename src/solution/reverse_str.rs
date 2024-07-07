//Write a function that reverses a string.

fn reverse(s: &mut String) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_reverse() {
        let mut s = String::from("najnarus");
        assert_eq!("suranjan", reverse(&mut s));
    }
}
