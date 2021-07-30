fn main() {
    let a = vec![1,2,3,4,5];
    println!("{}", sum_int(&a).unwrap());
}

fn sum_int(x: &[u32]) -> Option<u32> {
    let sum = x.iter().sum();
    if sum <= u32::MAX {
        return Some(sum)
    }
    None
}
