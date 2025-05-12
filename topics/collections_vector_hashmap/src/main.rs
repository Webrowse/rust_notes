// // Topic: Collections - Vectors and HashMaps

// use std::collections::HashMap;

// fn main() {
//     // ---------- VECTORS ----------
    // let mut numbers = vec![10, 20, 30];
//     numbers.push(40);
//     numbers.push(50);

//     println!("All numbers:");
//     for num in &numbers {
//         println!("{}", num);
//     }

//     // Access by index
//     println!("First: {}", numbers[0]); // 10

//     // Using get to avoid panic
//     match numbers.get(100) {
//         Some(num) => println!("Found: {}", num),
//         None => println!("Index out of bounds"),
//     }

//     // Remove last item
//     if let Some(last) = numbers.pop() {
//         println!("Removed last: {}", last); // 50
//     }

//     // ---------- HASHMAPS ----------
//     let mut scores = HashMap::new();

//     // Insert
//     scores.insert(String::from("Alice"), 50);
//     scores.insert(String::from("Bob"), 60);

//     // Overwrite
//     scores.insert(String::from("Alice"), 70);

//     // Access
//     let name = String::from("Alice");
//     match scores.get(&name) {
//         Some(score) => println!("{}'s score: {}", name, score), // 70
//         None => println!("No score for {}", name),
//     }

//     // Iterate
//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }

//     // Entry API - insert only if key is absent
//     scores.entry(String::from("Charlie")).or_insert(80);
//     scores.entry(String::from("Bob")).or_insert(100); // won't overwrite Bob's existing score

//     println!("Final scores:");
//     dbg!(&scores);
// }

//Exercises: 


// 1 . Create a vector of strings, add three items, remove the last, print all remaining.

// fn main(){
//     let mut vos = vec!["a", "b", "c"];
//     vos.push("d");
//     vos.push("e");
//     vos.push("f");

//     vos.pop();
//     println!("{:?}",vos);
// }

//2. Access vector index safely with .get() and handle out-of-bounds with match.

// fn main (){
//     let x = vec![2,4,6,8,10];
//     match x.get(4){
//         Some(x) => println!("{}",x),
//         None => println!("hehehe"),
//     };
//     match x.get(6){
//         Some(x) => println!("{}",x),
//         None => println!("hehehe"),
//     };
// }

// 3. Iterate over a vector with mutable references and increment each value by 1.

fn main(){
    let mut vec = vec![1,3,5,7,9];
    for x in &mut vec{
        *x +=1;
    }
    println!("{:?}",vec);
}