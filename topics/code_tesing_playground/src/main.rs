fn get_ref<'a>(s: &'a str) -> &'a str {
    s
}

fn main() {
    let s = String::from("temporary string");
    let r = get_ref(&s);
    println!("{}", r);
}
