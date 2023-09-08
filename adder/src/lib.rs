pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(n: i32) -> i32{
    n+2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    //#[test]
    //fn another() {
    //    assert_eq!(4,5);
    //}

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(3),5);
    }
}
