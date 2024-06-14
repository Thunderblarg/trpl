use rand;

pub fn add(left: usize, right: usize) -> usize {
    return left + right;
}

pub fn add_one(x: i32) -> i32 {
    return x + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
