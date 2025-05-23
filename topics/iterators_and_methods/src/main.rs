


fn main2() {
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
        println!("{}: {}", i+1, val);
    }
}

fn main(){

    // Exercises:
    // 1. Create a function that takes a `Vec<i32>`, prints all elements using `iter()`, and returns nothing.
    let vec = vec![12,13,45];
    for val in vec.iter(){
        println!("{}",val);
    }   
    // 2. Clone a vector and consume it with `into_iter()`, returning a new vector with all elements doubled.
    let vec2 = vec![12,6,31];
    for val in vec2.clone().into_iter(){
        println!("{}",val);
    }
    // 3. Use `iter_mut()` to modify a `Vec<i32>` in-place by squaring each element. Return the modified vector.
    let mut vec3 = vec![2,4,5,6,7];
    for val in vec3.iter_mut(){
        *val *= *val;
    }
    println!("{:?}",vec3);
    // 4. Use `map` and `collect` to convert a `Vec<&str>` to a `Vec<String>`.
    let vec4 = vec!["abc","def","ghi","jkl"];
    let mac:Vec<_> = vec4.iter().map(|x| x.to_string() ).collect();
    println!("{:?}",mac);
    // 5. Filter all even numbers from a vector using `filter` and `collect`. Return the new vector.
    let vec5 = vec![21,32,35,98];
    let even:Vec<_> = vec5.iter().filter(|&x|*x%2 == 0).collect();
    println!("{:?}",even);
    // 6. Implement a sum aggregator with `fold` that returns the product of all elements instead of the sum.
    let vec6 = vec![3,2,4,5,6,1];
    let prod = vec6.iter().fold(1 ,| acc,x|acc * *x);
    println!("{:?}",prod);
    // 7. Use `any` to check if any number in a vector is divisible by 7. Return the boolean result.
    let vec7 = vec![3,6,8,9,12,21];
    let any7 = vec7.iter().any(|x|x%7==0);
    println!("{:?}",any7);
    // 8. Use `all` to check if all elements in a vector are positive and less than 100. Return the boolean result.
    let vec8 = vec![2,56,82,1,34];
    let less_cent = vec8.iter().all(|x|*x<100);
    println!("{}",less_cent);
    // 9. Iterate over a vector with `enumerate` and build a `Vec<String>` of the form `"Index X: Value Y"`.
    let vec9 = vec!["abc".to_string(),"def".to_string(),"ghi".to_string()];
    let index_value: Vec<_> = vec9.iter()
    .enumerate()
    .map(|(i, val)|format!("Index {}: Value {}",i,val)).collect();
println!("{:?}",index_value);
    // 10. Combine `filter`, `map`, and `collect` to return the squares of all odd numbers in a vector.
    let vec10 = vec![3,4,5,8,10,11];
    let f_m_c: Vec<_> = vec10.iter().filter(|&x|*x%2==1).map(|x|x * x).collect();
    println!("{:?}",f_m_c);
    
}