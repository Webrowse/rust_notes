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

// fn main(){
//     let mut vec = vec![1,3,5,7,9];
//     for x in &mut vec{
//         *x +=1;
//     }
//     println!("{:?}",vec);
// }

// 4. Create a vector, remove elements with .pop() in a loop until empty, printing each removed item.
// fn main(){
//     let mut vec = vec![1,3,5,7,9];
//     while let Some(val) = vec.pop() {
//         println!("{}",val);
//     }
// }

// 5 . Create a Hashmap with three key-value pairs, then overwrite one key's value.

// use std::collections::HashMap;
// fn main(){
//     let mut data = HashMap::new();
//     data.insert(String::from("Adarsh"), 100);
//     data.insert(String::from("Babu"), 70);
//     data.insert(String::from("Captain"), 40);

//     println!("{:?}", data);

//     data.insert(String::from("Adarsh"), 121);

//     println!("{:?}", data);
// }

// 6. Use .get() on a HashMap for a key that might not exist, handle both outcomes.

// use std::collections::HashMap;
// fn main(){
//     let mut dummy = HashMap::new();
//     dummy.insert("romy", 3);
//     dummy.insert("adarsh", 23);


//     let index = "adarsh";
//     match dummy.get(&index){
//         Some(something) => println!("found: {}", something),
//         None => println!("ho"),
//     }
// }

// 7. Iterate over a Hashmap and print all key-value pairs.

// use std::collections::HashMap;

// fn main(){
//     let mut hm1  =  HashMap::new();
//     hm1.insert("a", 1);
//     hm1.insert("b", 2);
//     hm1.insert("c", 3);
//     hm1.insert("d", 4);
//     hm1.insert("e", 5);

    
//     for (k,v) in &hm1{
//         println!("{}, {}", k,v)
//     }
//     println!("{:?}",hm1);
// }

// 8. Use .entry().or_insert() to conditionally insert a new key in a Hashmap.

// use std::collections::HashMap;

// fn main(){
//     let mut x = HashMap::new();
//     x.insert("a",10);
//     x.insert("b",20);
//     x.insert("c",33);

//     println!("{:?}",x);

//     x.entry("c").or_insert(45);
//     x.entry("d").or_insert(35);

//     println!("{:?}",x);
// }

// 9 .Count occurrences of words in a string using a HashMap.
// use std::collections::HashMap;

// fn main(){
//     let x = "a b c a b f";
//     let y = x.split_whitespace();
//     // println!("{:?}",y);
//     let mut count: HashMap<&str,i32> = HashMap::new();
//     for a in y{
//         println!("{:?},",a);
//         *count.entry(a).or_insert(0) += 1;
//     }
//     println!("{:?}",count);
// }

// 10. Store and update scores for players in a game, where new players start with 0 and gain points.

use std::collections::HashMap;

fn counts<'a>(goal:&Vec<&'a str>) -> HashMap<&'a str,u8> {
    let mut hm: HashMap<&str,u8> = HashMap::new();
    for &color in goal{
        *hm.entry(color).or_insert(0)+=1;
    }
    hm
}

fn main(){
    let mut x = vec![];
    x.push("red");
    x.push("pink");
    x.push("blue");

    let hm = counts(&x);
    println!("{:?}",hm);
}