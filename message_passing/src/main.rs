//use std::sync::mpsc;
//use std::thread;
//use std::time::Duration;
use std::sync::{Arc,Mutex};
use std::thread;

fn main() {
    let v = Arc::new(Mutex::new(vec![1,2,3]));
    let mut children = vec![];

    for i in 0..3 {
      let cloned_v = v.clone();
      children.push(thread::spawn(move || {
         for j in 0..10000{
            cloned_v.lock().unwrap().push(i);
         }
      }));
    }
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

    print!("{:?}",v);
}
/*(
fn main() {
    let nsize=3;
    let nthreads=3; 
    let mut system = Arc::new(Mutex::new(vec![vec![1, 2, 3],vec![4,5,6],vec![7,8,9]]));

    for i in 0..3{
        thread::scope(|s| {
            for _ in 0..nthreads{
                s.spawn(|_| {
                    //let my_id: usize = rx_iter1.recv().unwrap();
                    let my_id=0;

                    for m in i..nsize{
                        if m % nthreads == my_id {
                            let mut mine = system[m].clone();
                            //change mine
                            mine[0]=0;
                            let mut shared = system.lock().unwrap();
                            shared[m]=mine;
                        }
                    }
                });
            }
        }).unwrap();
    }
}*/
