// Exercises
fn main2(){
    //1. Define a second struct Tweet with fields and implement Summary for it. Pass it to notify.
    //2. Create a function compare<T: PartialOrd>(a: T, b: T) that prints which value is greater. Test with integers and floats.
    //3. Modify Point<T> to implement a method swap() that swaps x and y. Require T: Clone.
    //4. Implement Summary for Point<String>, return a formatted coordinate string. Use notify on it.
    //5. Create a function print_summary<T: Summary>(item: &T) that uses a reference instead of ownership.
    //6. Add a second trait Displayable and implement both Summary and Displayable for Article. Write a function requiring T: Summary + Displayable.
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
    let post = Article {
        title: String::from("Rust Hits 2.0"),
        author: String::from("Adarsh"),
    };

    notify(post);

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.1, y: 2.2 };
    println!("p1: ({}, {}), p2: ({}, {})", p1.x, p1.y, p2.x, p2.y);
}
