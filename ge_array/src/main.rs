use gaussian_elimination;
use std::env;
use std::time;
fn main(){

    let args: Vec<String> = env::args().collect();

    let size :usize = args[1].parse().unwrap();
    let num_threads :u32 = args[2].parse().unwrap();

    let start_time = time::Instant::now(); 

    let system     = gaussian_elimination::init(size);
    let init_time  = start_time.elapsed().as_millis();

    let solutions  = gaussian_elimination::run(system,size,num_threads);
    let solve_time = start_time.elapsed().as_millis()-init_time;

    //for s in 0..size{
     //   print!("{}\n",solutions[s]);
    //}

    let err = gaussian_elimination::verify(solutions,size);
    print!("{},{},{},{},{}\n",size,num_threads,init_time,solve_time,err);

}
