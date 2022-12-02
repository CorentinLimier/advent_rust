#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_calories() {
        let calories = vec![1,2,3];
        let elf: Elf = Elf::new(calories);
        assert_eq!(elf.sum_calories(), 6)
    }
}