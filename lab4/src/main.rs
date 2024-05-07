use std::env;
use std::thread;
use std::sync::Arc;
mod models;
use models::{City, CityError};

fn main() -> Result<(), CityError>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No input file, usage: cargo run -- <file-path>");
    } 
    let city = City::config_city(args[1].as_str())?;

    // let (time_fedups, time_postnhl) = _montecarlo(&city);
    // let (time_fedups, time_postnhl) = _montecarlo_with_threads(city);
    let (time_fedups, time_postnhl) = average_t(city, 1000000);

    println!("FedUPS: {}", time_fedups);    
    println!("PostNHL: {}", time_postnhl);

    Ok(())
}

fn _montecarlo_t(city: &Arc<City>) -> (u32, u32) {

    let city_for_fedups = Arc::clone(&city);
    let handle_fed = thread::spawn(move || {
        city_for_fedups.find_path_montecarlo(city_for_fedups.get_start_a())
    });
    
    let city_for_postnhl = Arc::clone(&city);
    let handle_nhl = thread::spawn(move || {
        city_for_postnhl.find_path_montecarlo(city_for_postnhl.get_start_b())
    });

    let time_fedups = handle_fed.join().unwrap();
    let time_postnhl = handle_nhl.join().unwrap();

    (time_fedups, time_postnhl)
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


fn average_t(city: City, total_runs: u32) -> (f64, f64) {
    let city = Arc::new(city);
    let num_threads = 1000;
    let runs_per_thread = total_runs / num_threads;
    let mut handles = vec![];
    for _ in 0..num_threads {
        let city_clone = Arc::clone(&city); // Clone Arc, not the city itself
        let handle = thread::spawn(move || {
            run_montecarlo_chunk(&city_clone, runs_per_thread)
        });
        handles.push(handle);
    }
    let mut total_fedups: f64 = 0.0;
    let mut total_postnhl: f64 = 0.0;
    let mut i = 1;
    for handle in handles {
        let (fedups, postnhl) = handle.join().unwrap();
        println!("{} threads has finished!", i);
        total_fedups += fedups;
        total_postnhl += postnhl;
        i += 1;
    }

    (total_fedups / total_runs as f64, total_postnhl / total_runs as f64)

}



