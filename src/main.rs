pub mod planets;

use planets::{Planet, math::add};
use constants::*;

mod constants {
    pub const DIMENSION : usize = 2;
    pub const DT : f64 = 0.000001;
    pub const G : f64 = 6.674e-11;
    pub const NUM_PLANETS : usize = 3;
    pub const DURATION : i32 = 1000000000;
}

/*NOTES */
/** 
 *  Add a combine feature where planets that are too close become one larger planet
 * 
 *  Imlpement a graphical feature, boring otherwise
 * 
 *  Implement paralelism
 * 
 * 
 * 
 * **/

fn main() {
    let mut planets = initialize();
    let mut count = 0;
    //MAIN LOOP
    for _ in 0..DURATION {
        for i in 0..NUM_PLANETS {
            //planets[i].planet_update_routine(planets);
            let mut force;
            let mut total_force  = [0.0; DIMENSION];
            for j in 0..NUM_PLANETS {
                if i != j {
                    force = planets[i].find_force(&(planets[j]));
                    total_force = add(force, total_force);
                }
            }
    
            //Updating position and velocity
            planets[i].update_position();
            planets[i].update_velocity(total_force);
        }
        if count % 1000 == 0 {
            println!("0 pos [{:.2}, {:.2}] \t1 pos [{:.2}, {:.2}] \t2 pos [{:.2}, {:.2}]", 
                planets[0].position[0], 
                planets[0].position[1], 
                planets[1].position[0], 
                planets[1].position[1],
                planets[2].position[0], 
                planets[2].position[1],
            );
        }
        count += 1;
    }

}

pub fn initialize() -> [Planet; NUM_PLANETS] {

    //GOAL: implement a loop in here somehow

    let planet0 = Planet::new(
        0,
        100000000000.0,
        [10.0, 0.0],
        [0.0; DIMENSION],
    );

    let planet1 = Planet::new(
        1,
        100000000000.0,
        [-10.0, 0.0],
        [0.0; DIMENSION],
    );

    let planet2 = Planet::new(
        1,
        100000000000.0,
        [0.0, 10.0],
        [0.0; DIMENSION],
    );

    let out : [Planet; NUM_PLANETS]=  [planet0, planet1, planet2];

    out
}