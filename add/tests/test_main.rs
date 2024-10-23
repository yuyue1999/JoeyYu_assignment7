// #[cfg(test)]
use add::add;
mod tests {
    use super::*;
    fn test1() {
        assert_eq!(add(&2, &3), 5);
    }
    fn test2() {
        assert_eq!(add(&0, &-1), -1);
    }
    fn test3() {
        assert_eq!(add(&-1, &-10), -11);
    }
    fn test4() {
        assert_eq!(add(&10000, &1000), 11000);
    }
}