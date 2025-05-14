fn main() {
    let vec = vec![10, 20, 30];

    // iter() gives references
    for val in vec.iter() {
        println!("iter: {}", val);
    }

    // into_iter() takes ownership
    for val in vec.clone().into_iter() {
        println!("into_iter: {}", val);
    }

    // iter_mut() gives mutable refs
    let mut nums = vec![1, 2, 3];
    for val in nums.iter_mut() {
        *val *= 10;
    }
    println!("after iter_mut: {:?}", nums);

    // map and collect
    let doubled: Vec<_> = nums.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    // filter values >10
    let filtered: Vec<_> = nums.iter().filter(|&&x| x > 10).collect();
    println!("filtered: {:?}", filtered);

    // fold to sum all
    let sum = nums.iter().fold(0, |acc, x| acc + x);
    println!("sum: {}", sum);

    // any and all
    println!("any even: {}", nums.iter().any(|x| x % 2 == 0));
    println!("all > 0: {}", nums.iter().all(|x| *x > 0));

    // with index
    for (i, val) in nums.iter().enumerate() {
        println!("{}: {}", i, val);
    }
}
