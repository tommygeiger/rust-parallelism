use rand::Rng;
//use crossbeam;

use crossbeam::thread;
use crossbeam::crossbeam_channel::bounded;

//use std::thread;
//use thread_id;
//use std::sync::mpsc;
//use std::sync::Arc;

pub fn init(size:usize) -> Vec<Vec<f64>>{
    //init matrix
    let mut rng = rand::thread_rng();
    let mut system:Vec<Vec<f64>> = vec![vec![0.0;size+1];size];
    for i in 0..size{
        for j in 0..size{
            system[i][j] = rng.gen_range(0.0..10.0);
        }
    }

    //init R
    for i in 0..size{
        for j in 0..size{
            system[i][size] += (j + 1) as f64 * system[i][j];
        }
    }
    system
}

pub fn run(system: Vec<Vec<f64>>, size: usize, num_threads: usize) -> Vec<f64>{
    //solve
    eliminate(system,size,num_threads).unwrap()

}

pub fn verify(solution: Vec<f64>, size:usize) -> f64 {
    let mut err   = 0.0;
    let mut err__;
   // let mut worst_i = 0;
    for i in 0..size{
        let actual = i as f64 + 1.0;
        err__ = (solution[i] - actual).abs() / actual;
        if err < err__{
            err = err__;
    //        worst_i=i;
        }
    }
    //print!("Worst i: {}. solution[i]: {}\n",worst_i,solution[worst_i]);
    err
}

pub fn swap_rows(system: &mut Vec<Vec<f64>>, size: usize, a: usize , b: usize)  {
    let mut temp;
    for i in 0..size+1 {
        temp=system[a][i];
        system[a][i]=system[b][i];
        system[b][i]=temp;
    }
}
pub fn print_matrix(system: &Vec<Vec<f64>>, size: usize){
    for ip in 0..size{
        for jp in 0..size + 1{
            print!(" {}, ",system[ip][jp]);
        }
        print!("\n");
    }
        print!("===========\n");
}

pub fn eliminate(mut system: Vec<Vec<f64>>, size: usize, num_threads: usize) -> Option<Vec<f64>> {
    // produce the row reduced echelon form
    //
    // for every row...
    let mut pivotrow: usize;
    let mut pivotval;
    let mut x:Vec<f64> = vec![0.0;size];

    let (tx_data, rx_data) = bounded(size);
    let (tx_iter, rx_iter) = bounded(num_threads);
    for i in 0..size {
        //print_matrix(&system,size);
        
        pivotrow=i;
        for j in i..size-1 {
            if system[j][i].abs() > system[pivotrow][i].abs() {
                pivotrow=j;
            }
        }
        if pivotrow != i{
            swap_rows(&mut system,size,i,pivotrow);
        }

        //scale main row
        pivotval=system[i][i];
        if pivotval!=0f64{
            system[i][i]= 1.0;
            for j in i+1..size+1 {
                system[i][j]/=pivotval;
            }
        }

        for t in 0..num_threads{
            tx_iter.send(t).unwrap();
        }

        // for every column in that row...
        //println!("           {}",i);
        thread::scope(|s| {
            for _ in 0..num_threads {
                s.spawn(|_| {
                    let tx_data1 = tx_data.clone();
                    let rx_iter1 = rx_iter.clone();

                    let my_id: usize = rx_iter1.recv().unwrap();
                    //println!("{}: Received id", my_id);

                    for j in i+1..size {
                        //println!("{}: j = {}",my_id,j);
                        if j % num_threads == my_id{
                            //println!("{}: Processing line {}",my_id,j);
                            let mut mine = system[j].clone();
                            let factor = system[j][i];
                            // reduce every element under that element to 0
                            mine[i]= 0f64;
                            for k in i+1..size+1 {
                                mine[k] -= factor * system[i][k];
                            }

                            //println!("{}: Sending line {}",my_id,j);
                            tx_data1.send((mine,j)).unwrap();
                            //println!("{}: Sent line {}",my_id,j);
                        }
                    }
                });
            }
        }).unwrap();

        for _ in i+1..size{
            let (vec,index) = rx_data.recv().unwrap();
            //println!("Got {}: {:?}",index,vec);
            system[index]=vec;
        }
    }

    //print!("Matrix in REF: \n");
    //print_matrix(&system,size);

    x[size-1]=system[size-1][size];

    for i in (0..size-1).rev() {
        x[i] = system[i][size];
        for j in ((i+1)..size).rev() {
            x[i] -= system[i][j] * x[j];
        }
    }
    return Some(x);
}
 
#[cfg(test)]
mod tests {
    use super::*;
}
 
