use std::fmt::Display;


// Exercises
fn main2(){
    // 1. Define a second struct Tweet with fields and implement Summary for it. Pass it to notify.
    trait Summary{
        fn summarize(&self) -> String;
    }
    struct Tweet{
        content: String,
        name: String
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{} by: {}", self.content, self.name)
        }
    }
    fn notify<T: Summary>(item:T){
        println!("notify this: {}",item.summarize());
    }
    let tweet_work = Tweet{content: "the topic of tweet".to_string(), name: "Romy".to_string()};
    notify(tweet_work);
    //2. Create a function compare<T: PartialOrd>(a: T, b: T) that prints which value is greater. Test with integers and floats.
    fn compare<T: PartialOrd+std::fmt::Display>(a: T, b: T){
        if a > b {println!("{} is greater than {}",a,b)} else{println!("{} is greater than {}",b,a)}
    }
    compare(3, 5);
    compare(2.3, 2.4);
    //3. Modify Point<T> to implement a method swap() that swaps x and y. Require T: Clone.
    struct Point<T>{
        x: T,
        y: T,
    }
    trait Modify {
        fn swap(&mut self);
    }
    impl <T: Clone + Display> Modify for Point<T>{
        fn swap(&mut self) {
            println!("Before Swap: x = {} and y = {}",self.x,self.y);
            let temp = self.x.clone();
            self.x = self.y.clone();
            self.y = temp;
            println!("After Swap: x = {} and y = {}",self.x,self.y);
        }
    }
    fn swappy <T: Modify>(mut inp: T) {
        inp.swap();
    }
    let p = Point{x:10,y:12};
    swappy(p);

    //4. Implement Summary for Point<String>, return a formatted coordinate string. Use notify on it.
    impl<T: Display> Summary for Point<T>{
        fn summarize(&self) -> String {
            format!("The coordinates are ({}, {})",self.x,self.y)
        }
    }
    
    let ex4 = Point{
        x: "3.4332".to_string(),
        y: "7.5439".to_string()
    };
    notify(ex4);

    //5. Create a function print_summary<T: Summary>(item: &T) that uses a reference instead of ownership.
    fn print_summary<T: Summary>(item: &T){
        println!("print_summary: {}",item.summarize());
    }
    let prin_sum = Point{
        x: String::from("34°23'16'' N"),
        y: String::from("98°73'54'' S")
    };
    print_summary(&prin_sum);
    //6. Add a second trait Displayable and implement both Summary and Displayable for Article. Write a function requiring T: Summary + Displayable.
    trait Displayable{
        fn sometin(&self);
    }
    impl Displayable for Point<i32>{
        fn sometin(&self) {
            println!("double of ({}) is ({:#?})",self.x, self.x * 2);
        }
    }
    fn ex6 <T: Summary + Displayable>(item: T){
        item.summarize();
        item.sometin();
    }
    ex6(Point{x:5,y:9});

    //7. Create a trait Distance with method distance_from_origin(&self) -> f64. Implement for Point<f64>.
    //8. Make Point<T> have a method mixup<U>(self, other: Point<U>) -> Point<T> that creates a new point from self.x and other.y.
    //9. Write a generic function largest<T: PartialOrd + Copy>(list: &[T]) -> T. Test with arrays of numbers.
    //10. Implement default method in trait Summary called summarize_author(&self) -> String that returns "Unknown author" unless overridden. Override in Article.
}

// Define a trait (like an interface)
trait Summary {
    fn summarize(&self) -> String;
}

// Implement the trait for a struct
struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

// Generic function with trait bound
fn notify<T: Summary>(item: T) {
    println!("Breaking news: {}", item.summarize());
}

// Struct with generic type
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    main2();
    println!("*******************");
    let post = Article {
        title: String::from("Rust Hits 2.0"),
        author: String::from("Adarsh"),
    };

    notify(post);

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.1, y: 2.2 };
    println!("p1: ({}, {}), p2: ({}, {})", p1.x, p1.y, p2.x, p2.y);
}
