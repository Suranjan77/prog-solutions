fn find_max(arr: &Vec<i32>) -> i32 {
    *arr.iter().max().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_max() {
        assert_eq!(9, find_max(&vec![2, 5, 4, 6, 4, 7, 9, 8, 0]))
    }
}
