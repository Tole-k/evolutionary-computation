use evolutionary::hybrid_evolutionary;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use evolutionary::hybrid_evolutionary::test;

    use super::*;

    #[test]
    fn test_add() {
        hybrid_evolutionary::test();
    }
}
