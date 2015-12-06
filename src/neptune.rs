use angle;

/**
Returns Neptune's equatorial angular semidiameter

# Arguments

* ```distance_to_earth```: Neptune's distance from Earth (in AU)
**/
pub fn semidiameter(distance_to_earth: f64) -> f64 {
    angle::pure_degrees(0.0, 0.0, 33.5) / distance_to_earth
}