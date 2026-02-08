fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(10, 0), Err("Division by zero".to_string()));
    }

    #[test]
    #[should_panic(expected = "Divide by zero")]
    fn test_panic() {
        // This would panic in real code, but for demo
        panic!("Divide by zero");
    }

    #[test]
    fn test_result() {
        let result = divide(10, 2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }
}

fn main() {
    println!("Run 'cargo test' to execute the tests");
    println!("Add result: {}", add(5, 3));
}