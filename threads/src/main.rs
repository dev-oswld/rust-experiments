//Concurrent programming
use std::thread;
use std::time::Duration;

fn main() {
    let _one = thread::spawn(|| {
      for i in 1..10 {
        println!("1: Number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
      }
   });
   _one.join().unwrap();

   let _two = thread::spawn(|| {
        for i in 1..30 {
           println!("2: Number {} from the second thread!", i);
           thread::sleep(Duration::from_millis(1));
        }
    });
    _two.join().unwrap();
}