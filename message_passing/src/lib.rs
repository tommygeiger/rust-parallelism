//use std::sync::mpsc;
//use std::thread;
//use std::time::Duration;
use crossbeam::thread;

use crossbeam::crossbeam_channel::bounded;
fn main() {
    let nsize=3;
    let nthreads=3; 
    let mut system = vec![vec![1, 2, 3],vec![4,5,6],vec![7,8,9]];
    let (tx_data, rx_data) = bounded(nsize);
    let (tx_iter, rx_iter) = bounded(nthreads);

    println!("Main thread initialized values");


    for i in 0..3{
        println!("Main thread sending");
        for i in 0..nthreads{
            println!("{}",i);
            tx_iter.send(i).unwrap();
        }
        thread::scope(|s| {
            for _ in 0..nthreads{
                s.spawn(|_| {
                    println!("Child thread created");
                    let tx_data1 = tx_data.clone();
                    let rx_iter1 = rx_iter.clone();

                    println!("Waiting for id:");
                    let my_id: usize = rx_iter1.recv().unwrap();
                    println!("Received id");

                    for m in i..nsize{
                        if m % nthreads == my_id {
                            let mut mine = system[m].clone();
                            //change mine
                            tx_data1.send((mine,my_id)).unwrap();
                        }
                    }
                });
            }
        }).unwrap();

        println!("Main thread receiving");
        for _ in i..nsize{
            let (vec,index) = rx_data.recv().unwrap();
            println!("Got {}: {:?}",index,vec);
            system[index]=vec;
        }
    }
}
