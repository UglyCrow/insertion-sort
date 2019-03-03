fn main() {
    let vec = vec![3,1,4,1,5,9,2,6,5,3,5,8,9,7,9,3,2,3,8,4];
    let mut sorted: Vec<i32> = Vec::new();
    let len = vec.len();
    for target in 0..len {
        let tmp = vec[target];
        let mut insert_location = sorted.len();
        while insert_location != 0 && sorted[insert_location - 1] > tmp {
            insert_location -= 1;
        }
        sorted.insert(insert_location, tmp);
    }
    assert!(sorted == vec![1,1,2,2,3,3,3,3,4,4,5,5,5,6,7,8,8,9,9,9]);
    println!("{:?}", sorted);
}
