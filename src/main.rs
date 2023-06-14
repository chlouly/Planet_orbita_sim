pub mod planets;

use planets::*;
// use planets::math::DIM;
// use planets::math::DT;
// use planets::math::G;


fn main() {
    let mut planets = initialize();
    //MAIN LOOP
    for _ in 0..DURATION {
        for i in 0..NUM_PLANETS {
            (planets[i]).planet_update_routine(&planets);
        }
    }

}

pub fn initialize() -> [Planet; NUM_PLANETS] {
    let mut out : [Planet; NUM_PLANETS];

    //GOAL: implement a loop in here somehow

    let mut planet0 = Planet::new(
        0,
        1.0,
        [1000.0, 0.0],
        [0.0; DIMENSION],
    );

    let mut planet1 =         Planet::new(
        0,
        1.0,
        [-1000.0, 0.0],
        [0.0; DIMENSION],
    );

    out = [planet0, planet1];

    out
}