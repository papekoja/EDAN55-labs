use std::env;
use std::io;
use std::io::Write;
use std::thread;
use std::sync::Arc;

mod models;
mod menu;
use models::{City, CityError};

fn main() -> Result<(), CityError>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No input file, usage: cargo run -- <file-path>");
    } 
    let city = City::config_city(args[1].as_str())?;
    // println!("{:#?}", city);
    
    menu::print_menu();
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).map_err(|err| CityError::IoError(err))?;
    let choice = buffer.trim().parse::<usize>().map_err(|_| CityError::ParseIntError)?;

    menu(choice, city)?;

    Ok(())
}

fn menu(choice: usize, city: City) -> Result<(), CityError> {
    let time_fedups;
    let time_postnhl;
    match choice {
        1 => {
            time_fedups = city.find_path_montecarlo(city.get_start_a()) as f64;
            time_postnhl = city.find_path_montecarlo(city.get_start_b()) as f64;
        },
        2 => {
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!("Enter number of runs: ");
            io::stdout().flush().map_err(|err| CityError::IoError(err))?;
            stdin.read_line(&mut buffer).map_err(|err| CityError::IoError(err))?;
            let runs = buffer.trim().parse::<u32>().map_err(|_| CityError::ParseIntError)?;

            let (total_fedups, total_postnhl) = calculate_average(city, runs);
            time_fedups = total_fedups;
            time_postnhl = total_postnhl;
        },
        3 => {
            time_fedups = city.find_path_markov(city.get_start_a()) as f64;
            time_postnhl = city.find_path_markov(city.get_start_b()) as f64;
        },
        _ => {
            eprintln!("Unknown input.");
            return Err(CityError::InvalidInput);
        },
    }
    println!("FedUPS: {}", time_fedups);    
    println!("PostNHL: {}", time_postnhl);
    Ok(())
}

fn run_montecarlo_chunk(city: &City, runs: u32) -> (f64, f64) {
    let mut fedups_total_runtime = 0;
    let mut postnhl_total_runtime = 0;
    for _ in 0..runs {
        let fedup_runtime = city.find_path_montecarlo(city.get_start_b());
        let postnhl_runtime = city.find_path_montecarlo(city.get_start_a());
        fedups_total_runtime += fedup_runtime;
        postnhl_total_runtime += postnhl_runtime;
    }
    (fedups_total_runtime as f64, postnhl_total_runtime as f64)
}


fn calculate_average(city: City, total_runs: u32) -> (f64, f64) {
    
    let city = Arc::new(city);
    let num_threads = if total_runs < 1000 { 1 } else { 1000 };
    let runs_per_thread = total_runs / num_threads;
    let mut handles = Vec::with_capacity(num_threads as usize);
    
    for _ in 0..num_threads {
        let city_clone = Arc::clone(&city); // Clone Arc, not the city itself
        handles.push(thread::spawn(move || {
            run_montecarlo_chunk(&city_clone, runs_per_thread)
        }));
    }

    let mut total_fedups: f64 = 0.0;
    let mut total_postnhl: f64 = 0.0;
    let mut runs_completed = 0;

    for (i, handle) in handles.into_iter().enumerate() {
        let (fedups, postnhl) = handle.join().unwrap();

        total_fedups += fedups;
        total_postnhl += postnhl;
        runs_completed += runs_per_thread;
        if (i + 1) % 10 == 0 && num_threads > 1 {
            println!("{} threads has finished!", i + 1);
            println!("fedups:  {:.2}", (total_fedups as f64 / runs_completed as f64));
            println!("postnhl: {:.2}", (total_postnhl as f64/ runs_completed as f64));
            println!("-----------");
        }
    }

    (total_fedups / total_runs as f64, total_postnhl / total_runs as f64)

}



