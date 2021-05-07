use rand::Rng;
use std::thread;

pub fn init(size:usize) -> Vec<Vec<f64>>{
    //init matrix
    let mut rng = rand::thread_rng();
    let mut system:Vec<Vec<f64>> = vec![vec![0.0;size+1];size];
    for i in 0..size{
        for j in 0..size{
            system[i][j] = rng.gen_range(0.0..10.0);
            //system[i][j] = 10.0 * i as f64 + j as f64;
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

pub fn run(system: Vec<Vec<f64>>, size: usize, num_threads: u32) -> Vec<f64>{
    //solve
    eliminate(system,size).unwrap()

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

pub fn eliminate(mut system: Vec<Vec<f64>>, size: usize) -> Option<Vec<f64>> {
    // produce the row reduced echelon form
    //
    // for every row...
    let mut pivotrow: usize;
    let mut pivotval;
    let mut x:Vec<f64> = vec![0.0;size];
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
        // for every column in that row...
        for j in i+1..size {
            let factor = system[j][i];
            // reduce every element under that element to 0
            system[j][i]= 0f64;
            for k in i+1..size+1 {
                system[j][k] -= factor * system[i][k];
            }
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
 
