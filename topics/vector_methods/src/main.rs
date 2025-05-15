// fn main() {
//     let mut nums = vec![10, 20, 30];

//     // push adds at end
//     nums.push(40);
//     println!("after push: {:?}", nums); // [10, 20, 30, 40]

//     // pop removes last
//     nums.pop();
//     println!("after pop: {:?}", nums); // [10, 20, 30]

//     // insert at index
//     nums.insert(1, 15);
//     println!("after insert: {:?}", nums); // [10, 15, 20, 30]

//     // remove at index
//     nums.remove(2);
//     println!("after remove: {:?}", nums); // [10, 15, 30]

//     // check if contains
//     println!("contains 15? {}", nums.contains(&15)); // true

//     // get returns Option
//     println!("at 0: {:?}", nums.get(0)); // Some(10)
//     println!("at 100: {:?}", nums.get(100)); // None

//     // len and is_empty
//     println!("len: {}", nums.len()); // 3
//     println!("empty? {}", nums.is_empty()); // false

//     // clear removes all
//     nums.clear();
//     println!("after clear: {:?}", nums); // []
// }

 fn main(){
// 1. Create a vector with initial values `[1, 2, 3]`. Push `4`, `5`, then print the vector.
let mut vec = vec![1,2,3];
vec.push(4);
vec.push(5);
println!("{:?}",vec);
// 2. Start with `[10, 20, 30, 40]`. Pop twice. Print the vector and its length.
let mut vec2 = vec![10,20,30,40];
vec2.pop();
vec2.pop();
println!("{:?}",vec2);
// 3. Given `[100, 200, 300]`, insert `150` at index `1`. Remove the value at index `2`. Print final vector.
let mut vec3 = vec![100,200,300];
vec3.insert(1,150);
println!("{:?}",vec3);
// 4. Check if a vector `[5, 10, 15]` contains `10` and `20`. Print the boolean results.
let  vec4 = [5,10,15];
println!("contains '10', '20': {}",vec4.contains(&10));
println!("contains '10', '20': {}",vec4.contains(&20));
// 5. Use `.get()` to access index `0` and `5` of `[7, 8, 9]`. Print the results.
let vec5 = vec![7,8,9];
println!("index of 1: {:?}",vec5.get(1));
println!("index of 1: {:?}",vec5.get(5));
// 6. Start with an empty `Vec<String>`. Check and print whether it’s empty, then push `"hello"` and `"world"`. Check `is_empty` again.

// 7. Create a `Vec<char>` with `['a', 'b', 'c']`. Clear the vector. Print the result and check length.
// 8. Modify `[9, 8, 7]` using insert and remove to make it `[9, 10, 8]`. Print result.
// 9. Push integers 1 to 100 into a vector using a loop. Then pop 10 times. Print length and last value.
// 10. Create a vector, perform mixed operations (push, insert, remove, get, contains, len, clear) in sequence. Print after each step.

 }