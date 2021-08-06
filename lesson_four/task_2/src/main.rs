fn main() {
    let a = vec![1,2,3,4,5,u32::MAX];
    println!("{}", sum_int(&a).unwrap());
}

fn sum_int(x: &[u32]) -> Option<u32> {
//    let sum = x.iter().sum()?;
    match x.iter().sum() {
        Ok(s) => Some(s),
        Err(_) => None,
    }
//    if sum <= u32::MAX {
//        return Some(sum)
//    }
//    None
}
