use std::io;
use rand::distributions::{
        Distribution, 
        WeightedIndex
    };
use rand::prelude::thread_rng;
use nalgebra::{self, DMatrix};

#[derive(Debug)]
pub enum CityError {
    MissingParameter(&'static str),
    InvalidRange { param: &'static str, value: usize, min: usize, max: usize },    
    ParseIntError,
    ParseFloatError,
    IoError(io::Error),
    InvalidInput,
}

#[derive(Debug, Clone)]
pub struct City {
    road_system: Vec<Intersection>,
    prob_matrix: DMatrix<f64>,
    travel_matrix: DMatrix<u32>,
    config: CityConfig,
}

#[derive(Debug, Clone)]
pub struct Road {
    intersection_id:  usize,
    travel_time:        u32,
    probability:        f64,
}

impl Road {
    pub fn new(
        intersection_id:  usize,
        travel_time:        u32,
        prob:        f64,
    ) -> Self {
        Self {
            intersection_id:    intersection_id,
            travel_time:            travel_time,
            probability:                   prob,
        }
    } 
}

#[derive(Debug, Clone)]
pub struct Intersection {
    roads: Vec<Road>,
}

impl Intersection {
    pub fn new() -> Self {
        Self {
            roads: Vec::new(),
        }
    }
    pub fn add_road(&mut self, road: Road) {
        self.roads.push(road);
    }
    
}



#[derive(Debug, Clone)]
pub struct CityConfig {
    num_intersections: usize,
    num_roads: usize,
    end_intersection: usize,
    start_a: usize,
    start_b: usize,
}

impl CityConfig {
    fn parse_config(contents: &str) -> Result<Self, CityError> {
        let numbers = contents
            .split_whitespace()
            .map(|num| num.parse::<usize>().map_err(|_| CityError::ParseIntError))
            .collect::<Result<Vec<_>, _>>()?;
        
        if numbers.len() != 5 {
            return Err(CityError::MissingParameter("Expected exactly five integers"));
        }

        let (num_intersections, 
             num_roads, 
             end_intersection, 
             start_a, 
             start_b) = (numbers[0], numbers[1], numbers[2], numbers[3], numbers[4]);
        
        let config = Self {
            num_intersections,   // N
            num_roads,           // M
            end_intersection,    // H
            start_a,             // F
            start_b,             // P
        };
     
        CityConfig::validate_config(&config)?;
        Ok(config)
    }
    
    fn validate_config(config: &CityConfig) -> Result<(), CityError> {
        if config.num_intersections < 3 || config.num_intersections > 300 {
            return Err(CityError::InvalidRange { param: "number of intersections", value: config.num_intersections, min: 3, max: 300 });
        }

        let min_num_roads = config.num_intersections / 2;
        let max_num_roads = config.num_intersections * (config.num_intersections - 1) / 2;
        
        if min_num_roads > config.num_roads || config.num_roads > max_num_roads {
            return Err(CityError::InvalidRange { param: "number of roads", value: config.num_intersections, min: min_num_roads, max: max_num_roads });
        }

        if config.end_intersection > config.num_intersections {
            return Err(CityError::InvalidRange { param: "end intersection", value: config.end_intersection, min: std::usize::MIN, max: config.num_intersections })
        }

        if config.start_a > config.num_intersections {
            return Err(CityError::InvalidRange { param: "start a", value: config.end_intersection, min: std::usize::MIN, max: config.num_intersections })
        }
        if config.start_b > config.num_intersections {
            return Err(CityError::InvalidRange { param: "start b", value: config.end_intersection, min: std::usize::MIN, max: config.num_intersections })
        }

        Ok(())
    }
}


impl City {
    pub fn new(config: CityConfig, prob_matrix: DMatrix<f64>, travel_matrix: DMatrix<u32>) -> Self {
        Self {
            road_system: vec![Intersection::new(); config.num_intersections],
            prob_matrix: prob_matrix,
            travel_matrix: travel_matrix,
            config: config,
        }
    }

    pub fn config_city(file_path: &str) -> Result<City, CityError> {
        let contents = std::fs::read_to_string(file_path)
            .map_err(CityError::IoError)?;

        let mut lines = contents.lines();
        let config = CityConfig::parse_config(
            lines.next()
            .ok_or_else(|| CityError::MissingParameter("configuration line"))?)?;

        let prob_matrix: DMatrix<f64> = DMatrix::zeros(config.num_intersections, config.num_intersections);
        let travel_matrix: DMatrix<u32> = DMatrix::zeros(config.num_intersections, config.num_intersections);

        let mut city: City = Self::new(config, prob_matrix, travel_matrix); 

        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 5 {
                return Err(CityError::MissingParameter("Excepted five values per road line"))
            }

            let intersect_id_u = parts[0].parse::<usize>().map_err(|_| CityError::ParseIntError)?;
            let intersect_id_v = parts[1].parse::<usize>().map_err(|_| CityError::ParseIntError)?;
            let travel_time      = parts[2].parse::<u32>().map_err(  |_| CityError::ParseIntError)?;
            let prob_u_to_v      = parts[3].parse::<f64>().map_err(  |_| CityError::ParseFloatError)?;
            let prob_v_to_u      = parts[4].parse::<f64>().map_err(  |_| CityError::ParseFloatError)?;

            let road_u_to_v = Road::new(intersect_id_v, travel_time, prob_u_to_v);
            let road_v_to_u = Road::new(intersect_id_u, travel_time, prob_v_to_u);
            
            city.add_road(intersect_id_u, intersect_id_v, road_u_to_v);
            city.add_road(intersect_id_v, intersect_id_u, road_v_to_u);
        }
        Ok(city)
    }

    pub fn add_road(&mut self, id_u: usize, id_v: usize, road: Road) {
        self.prob_matrix[(id_u, id_v)] = road.probability;
        self.travel_matrix[(id_u, id_v)] = road.travel_time;
        self.road_system[id_u].add_road(road)
    }

    pub fn get_start_a(&self) -> usize {
        self.config.start_a
    }
    
    pub fn get_start_b(&self) -> usize {
        self.config.start_b
    }

}



impl City {
    // Algorithms
    pub fn find_path_markov(&self, start: usize) -> f64 {
        let dim = self.config.num_intersections;
        
        let a: &DMatrix<f64> = &self.prob_matrix;
        let t: DMatrix<f64> = self.travel_matrix.map(|n| n as f64);
        let i: DMatrix<f64> = DMatrix::<f64>::identity(dim, dim);
        let at_product = a.component_mul(&t);
        let b = at_product.column_sum();
        
        let a_minus_i = a - &i;
        
        if let Some(inv_a_minus_i) = a_minus_i.try_inverse() {
            let neg_b = -&b;
            
            let neg_b_matrix = DMatrix::<f64>::from_column_slice(dim, 1, neg_b.as_slice());
            let x = inv_a_minus_i * neg_b_matrix;                
            x[start]
            
        } else {
            f64::NAN
        }
           
    }

    pub fn find_path_montecarlo(&self, start: usize) -> u32 {
        let mut total_time = 0;
        let mut current_intersection = start;
        let mut rng = thread_rng();
        while current_intersection != self.config.end_intersection {
            let current = &self.road_system[current_intersection];

            if current.roads.is_empty() {
                eprintln!("no roads?");
                break; 
            }

            let weights: Vec<_> = current.roads.iter().map(|road| road.probability).collect();
            let dist = WeightedIndex::new(&weights).unwrap();
            let chosen_road = &current.roads[dist.sample(&mut rng)];
            
            total_time += chosen_road.travel_time;

            if chosen_road.intersection_id == current_intersection {
                eprintln!("loopback?");
                break;
            }

            current_intersection = chosen_road.intersection_id;
        } 

        total_time
    }

}