pub mod math;

use math::*;
use crate::constants::*;

pub struct Planet {
    pub planet_id : i32,
    pub mass : f64,
    pub position : [f64; DIMENSION],
    pub velocity : [f64; DIMENSION],
    pub path : Vec<[f64; DIMENSION]>,
}

impl Planet {
    pub fn new(pid : i32, m : f64, position : [f64; DIMENSION], velocity : [f64; DIMENSION]) -> Self {
        let new_planet = Self {
            planet_id : pid,
            mass : m,
            path : Vec::new(),
            position : position,
            velocity : velocity,
        };

        new_planet
    }

    // pub fn planet_update_routine(&mut self, all_planets : [Planet; NUM_PLANETS]) {
    //     //Finding total force
    //     let mut total_force  = [0.0; DIMENSION];
    //     let mut temp : [f64; DIMENSION];
    //     for i in 0..NUM_PLANETS {
    //         if (*all_planets)[i].planet_id != (*self).planet_id {
    //             temp = (*self).find_relative_pos(&((*all_planets)[i]));
    //             temp = (*self).find_force(&((*all_planets)[i]));
    //             total_force = add(total_force, temp);
    //         }
    //     }

    //     //Updating position and velocity
    //     (*self).update_position();
    //     (*self).update_velocity(total_force);
    // }

    pub fn find_relative_pos(&self, other_planet : &Planet) -> [f64; DIMENSION] {
        sub((*other_planet).position, (*self).position)
    }

    //Physicsy stuff
    pub fn fill_force_vector() {
         
    }

    pub fn find_force(&self, other_planet : &Planet) -> [f64; DIMENSION] {
        //With vectors, F = r_hat * GmM/(r_mag ^ 2) = r * GmM/(r_mag ^ 3)
        //Pushes that force onto Self's 'force' field vector
        let r = (*self).find_relative_pos(other_planet);
        let r_mag = find_mag(r);
        let factor = G * self.mass * (*other_planet).mass / (r_mag * r_mag * r_mag);

        mult(r, factor)
    }

    pub fn update_position(&mut self) {
        //WORKS
        let distance_traveled = mult(self.velocity, DT);
        let new = add(self.position, distance_traveled);
        (*self).position = new;

        //IF TRACKING PATH
        //self.path.push(new)
    }

    pub fn update_velocity(&mut self, total_force : [f64; DIMENSION]) {
        //WORKS
        let scalefactor : f64 = DT / (*self).mass;
        let mut temp = mult(total_force, scalefactor);
        temp = add((*self).velocity, temp);
        
        (*self).velocity = temp;
    }
}

