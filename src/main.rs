pub mod planets;

use planets::{Planet, math::add};
use constants::*;

mod constants {
    pub const DIMENSION : usize = 2;
    pub const DT : f64 = 0.000001;
    pub const G : f64 = 6.674e-11;
    pub const NUM_PLANETS : usize = 2;
    pub const DURATION : i32 = 1000000000;
    pub const IGNORE_THRESHOLD : f64 = 0.1;
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


    //MAIN LOOP
    for count in 0..DURATION {
        for i in 0..NUM_PLANETS {
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
        for i in 0..NUM_PLANETS {
            planets[i].swap_positions();
        }

        //printing planet's position
        if count % 1000 == 0 {print_pos(&planets);}
    }

}


pub fn initialize() -> [Planet; NUM_PLANETS] {

    //GOAL: implement a loop in here somehow

    let planet0 = Planet::new(
        0,
        1000000000000.0,
        [10.0, 0.0],
        [1.0, 0.0],
    );

    let planet1 = Planet::new(
        1,
        1000000000000.0,
        [-10.0, 0.0],
        [-1.0, 0.0],
    );

    // let planet2 = Planet::new(
    //     1,
    //     100000000000.0,
    //     [0.0, 10.0],
    //     [0.0; DIMENSION],
    // );

    let out : [Planet; NUM_PLANETS]=  [planet0, planet1];

    out
}


//Pritning to command line
pub fn print_pos(planets : &[Planet; NUM_PLANETS]) {

    for i in 0..NUM_PLANETS {
        print!("{} ", i);
        print_planet_pos(&(*planets)[i]);
    }
    print!("\n");
}

pub fn print_planet_pos(p : &Planet) {
    print!("pos [");
    for i in 0..DIMENSION {
        print!("{:.2}", (*p).position[i]);
        if i != (DIMENSION - 1) {
            print!(", ");
        }
    }
    print!("]\t");
}




