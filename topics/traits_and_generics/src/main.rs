use std::fmt::Display;
use std::clone::Clone;
use std::net::ToSocketAddrs;


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
    struct Point<T: Clone>{
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
    impl<T: Display + std::clone::Clone> Summary for Point<T>{
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
    trait Distance{
        fn distance_from_origin(&self) -> f64;
    }
    impl Distance for Point<f64>{
        fn distance_from_origin(&self) -> f64 {
            ((self.x).powi(2) + (self.y).powi(2)).sqrt()
        }
    }
    fn exercise7<T:Distance>(inp: T){
        println!("Implemented : {:.2}",inp.distance_from_origin());
    }
    // let point_first = Point{
    //     x: 3.45,
    //     y: 7.34
    // };
    // let point_second = Point{
    //     x: 8.25,
    //     y: 1.34
    // };
    // fn dist(s:&Point<f64>, t:&Point<f64>) -> f64{
    //     let sqnum = (s.x - t.x) * (s.x - t.x) + (s.y - t.y) * (s.y - t.y);
    //     sqnum.sqrt()
    // }
    // println!("{}",dist(&point_first, &point_second));
    let point_from_origin:Point<f64> = Point { x: 3.4, y: 6.8 };
    exercise7(point_from_origin);
    //8. Make Points<T> have a method mixup<U>(self, other: Points<U>) -> Points<T> that creates a new point from self.x and other.y.
    #[derive(Debug)]
    struct Points<T>{
        x: T,
        y: T
    }
    impl<T> Points<T>{
        fn mixup<U>(self, other: Points<U>) -> Points<T> where U: Into<T>{
            Points { 
                x: self.x, 
                y: other.y.into()
            }
        }
    }
    
    let p1= Points{
        x: 5,
        y: 10
    };
    let p2= Points{
        x: 2,
        y: 8
    };
    
    println!("{:?}",p1.mixup(p2));
    //9. Write a generic function largest<T: PartialOrd + Copy>(list: &[T]) -> T. Test with arrays of numbers.
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        list.iter().copied().fold(list[0],|acc, item| if acc > item { acc }else{item})
    }
    let exv9 = vec![2,1,3,4];
    println!("{}",largest(&exv9));
    //10. Implement default method in trait Summary called summarize_author(&self) -> String that returns "Unknown author" unless overridden. Override in Article.
    trait SummaryWithAuthor{
        fn summary(&self) -> String;
        fn summarize_author(&self) -> String{
            "Unknown Author".to_string()
        }
    }
    struct NewArticle{
        title: String,
        author: String
    }
    impl SummaryWithAuthor for NewArticle{
        fn summary(&self) -> String {
            // let n = "".to_string();
            let author = if self.author.is_empty() {self.summarize_author()} else {self.author.clone()};
            format!("post title: {} is written by: {}",self.title,author)
        }
    }
    fn lets_connect<T:SummaryWithAuthor>(inp:T){
        println!("{}",inp.summary());
    }
    let newPost = NewArticle{
        title: "My Article".to_string(),
        author: " ".trim().to_string(),
    };
    lets_connect(newPost);


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
