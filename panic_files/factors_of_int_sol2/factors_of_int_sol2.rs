fn main() {
    assert_eq!(vec![1, 2, 4, 5, 10, 10, 20, 25, 50, 100], factor(100)); // asserts that two expressions are equal to each other
    assert_eq!(vec![1, 101], factor(101));
 
}
 
fn factor(n: i32) -> Vec<i32> {
    (1..=n).filter(|i| n % i == 0).collect()
}