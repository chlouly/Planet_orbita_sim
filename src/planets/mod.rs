mod math;

use math::*;

pub const DIMENSION : usize = 2;
pub const DT : f32 = 1.0;
pub const G : f32 = 6.674e-11;
pub const NUM_PLANETS : usize = 2;
pub const DURATION : i32 = 1000;

pub struct Planet {
    pub planetID : i32,
    pub mass : f32,
    pub position : [f32; DIMENSION],
    pub velocity : [f32; DIMENSION],
    pub path : Vec<[f32; DIMENSION]>,
}

impl Planet {
    pub fn new(pid : i32, m : f32, position : [f32; DIMENSION], velocity : [f32; DIMENSION]) -> Self {
        let new_planet = Self {
            planetID : pid,
            mass : m,
            path : Vec::new(),
            position : position,
            velocity : velocity,
        };

        new_planet
    }

    pub fn planet_update_routine(&mut self, all_planets : &[Planet; NUM_PLANETS]) {
        //Finding total force
        let mut total_force  = [0.0; DIMENSION];
        let mut temp : [f32; DIMENSION];
        for i in 0..NUM_PLANETS {
            if (*all_planets)[i].planetID != (*self).planetID {
                temp = (*self).find_relative_pos(&((*all_planets)[i]));
                temp = (*self).find_force(&((*all_planets)[i]));
                total_force = add(total_force, temp);
            }
        }

        //Updating position and velocity
        (*self).update_position();
        (*self).update_velocity(total_force);
    }

    pub fn find_relative_pos(&self, other_planet : &Planet) -> [f32; DIMENSION] {
        sub((*other_planet).position, (*self).position)
    }

    //Physicsy stuff
    pub fn fill_force_vector() {
         
    }

    pub fn find_force(&self, other_planet : &Planet) -> [f32; DIMENSION] {
        //With vectors, F = r_hat * GmM/(r_mag ^ 2) = r * GmM/(r_mag ^ 3)
        //Pushes that force onto Self's 'force' field vector
        let mut r = (*self).find_relative_pos(other_planet);
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

    pub fn update_velocity(&mut self, total_force : [f32; DIMENSION]) {
        //WORKS
        let scalefactor : f32 = DT / (*self).mass;
        let mut temp = mult(total_force, scalefactor);
        temp = add((*self).velocity, temp);
        
        (*self).velocity = temp;
    }
}

