#[allow(dead_code)]
pub fn famous_saying(arr: &[&str]) {
    for i in 0..arr.len() - 1 {
        println!("For want of a {0} the {1} was lost.", arr[i], arr[i + 1]);
    }
}
