fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", array_diff(vec![1,2], vec![1])); // should print [2]
    println!("{:?}", array_diff(vec![1,2,2,2,3], vec![2])); // should print [1,3]
}