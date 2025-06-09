use std::{thread, time::Duration};


fn main(){
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        println!("Done from thread.");
    });

}
