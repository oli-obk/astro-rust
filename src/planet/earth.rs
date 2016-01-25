//! The Earth

use coords;
use angle;
use time;
use planet;

/**
Returns the **flattening factor** of the Earth

Reference: [World Geodetic System 1984](https://confluence.qps.nl/pages/viewpage.action?pageId=29855173)
**/
pub fn flat_fac() -> f64 {
    1.0 / 298.257223563
}

/**
Returns the **equatorial radius** of the Earth *| in kilometers*

Reference: [World Geodetic System 1984](https://confluence.qps.nl/pages/viewpage.action?pageId=29855173)
**/
pub fn eq_rad() -> f64 {
    6378.137
}

/**

Returns the **polar radius** of the Earth *| in kilometers*

Calculated using [```FlatteningFactor()```](./fn.FlatteningFactor.html) and
[```eq_radius()```](./fn.eq_radius.html)
**/
pub fn pol_rad() -> f64 {
    eq_rad() * (1.0 - flat_fac())
}

/**
Returns the **eccentricity** of the Earth's **meridian**

**/
pub fn ecc_of_meridian() -> f64 {
    ((2.0 - flat_fac()) * flat_fac()).sqrt()
}

/**
Returns a **low accuracy geodesic distance** between two points on the Earth's
surface *| in kilometers*

Assumes that the Earth is a sphere.

# Arguments

* ```p1```: ```GeographPoint``` 1
* ```p2```: ```GeographPoint``` 2
**/
pub fn approx_geodesic_dist(p1: &coords::GeographPoint, p2: &coords::GeographPoint) -> f64 {
    6371.0 * p1.angl_sepr(&p2)
}

/**
Returns a **high accuracy geodesic distance** between two points on the Earth's
surface *| in kilometers*

# Arguments

* ```p1```: ```GeographPoint``` 1
* ```p2```: ```GeographPoint``` 2
**/
pub fn geodesic_dist(p1: &coords::GeographPoint, p2: &coords::GeographPoint) -> f64 {
    let f = (p1.lat + p2.lat) / 2.0;println!("{:?}", f.to_degrees());
    let g = (p1.lat - p2.lat) / 2.0;println!("{:?}", g.to_degrees());
    let lam = (p1.long - p2.long) / 2.0;println!("{:?}", lam.to_degrees());
    let s = (g.sin() * lam.cos()).powi(2) +
            (f.cos() * lam.sin()).powi(2);println!("{:?}", s);
    let c = (g.cos() * lam.cos()).powi(2) +
            (f.sin() * lam.sin()).powi(2);println!("{:?}", c);
    let om = ((s / c).sqrt()).atan();println!("{:?}", om);
    let r = (s * c).sqrt() / om;println!("{:?}", r);
    let d = 2.0 * om * eq_rad();println!("{:?}", d);
    let h1 = (3.0*r - 1.0) / (2.0 * c);println!("{:?}", h1);
    let h2 = (3.0*r + 1.0) / (2.0 * s);println!("{:?}", h2);

    d * (  1.0
         + flat_fac() * h1 * (f.sin() * g.cos()).powi(2)
         - flat_fac() * h2 * (f.cos() * g.sin()).powi(2)
        )
}

/**
Returns two quantities that are used elsewhere in the library

# Returns

```Rho``` here denotes the distance from the Earth's center to a point
on the ellipsoid, and ```Phi``` here denotes the geocentric latitude,
both of an observer on the Earth's surface.

```rho_sin_phi, rho_cos_phi```

* ```rho_sin_phi```: Rho sin phi
* ```rho_cos_phi```: Rho cos phi

# Arguments

* ```geograph_lat```: Observer's geographic latitude *| in radians*
* ```height```: Observer's height above sea level *(meters)*
**/
pub fn rho_sin_cos_phi(geograph_lat: f64, height: f64) -> (f64, f64) {
    let u = (geograph_lat.tan() * pol_rad() / eq_rad()).atan();
    let x = height / (eq_rad() * 1000.0);
    let rho_sin_phi = (u.sin() * pol_rad() / eq_rad()) +
                      (geograph_lat.sin() * x);
    let rho_cos_phi = u.cos() + (geograph_lat.cos() * x);

    (rho_sin_phi, rho_cos_phi)
}

/**
Returns the **distance** from the Earth's **center to a point
on the ellipsoid**

# Returns

* ```rho```: Distance from the Earth's center to the point on the
             ellipsoid (*fraction of the equatorial radius*)

# Arguments

* ```geograph_lat```: Geographic latitude of a point on the ellipsoid *| in radians*
**/
pub fn rho(geograph_lat: f64) -> f64 {
      0.9983271
    + 0.0016764 * (2.0*geograph_lat).cos()
    - 0.0000035 * (4.0*geograph_lat).cos()
}

/// Returns the **rotational angular velocity** of the Earth *| in radian per second*
pub fn rot_angular_velocity() -> f64 {
    0.00007292114992
}

/**
Returns the **radius of the parallel of a latitude**

# Returns

* ```rad```: Radius of the parallel of the latitude *| in kilometers*

# Arguments

* ```geograph_lat```: Geographic latitude of a point on the ellipsoid *| in radians*
**/
pub fn rad_of_parll_lat(geograph_lat: f64) -> f64 {
    let e = ecc_of_meridian();
      eq_rad() * geograph_lat.cos()
    / (1.0 - ((e*geograph_lat.sin()).powi(2))).sqrt()
}

/**
Returns the **linear velocity of a point at a latitude**

# Returns

* ```lin_vel```: Linear velocity at the latitude (kilometers per second*)

# Arguments

* ```geograph_lat```: Geographic latitude of a point on the ellipsoid *| in radians*
**/
pub fn linear_velocity_at_lat(geograph_lat: f64) -> f64 {
    rot_angular_velocity() * rad_of_parll_lat(geograph_lat)
}

/**
Returns the **radius of curvature** of the Earth's **meridian
at a latitude**

# Returns

* ```rad```: Radius of curvature of the Earth's meridian at the
latitude *| in kilometers*

# Arguments

* ```geograph_lat```: Geographic latitude of a point on the ellipsoid *| in radians*
**/
pub fn rad_curv_of_meridian(lat: f64) -> f64 {
    let e = ecc_of_meridian();
      eq_rad() * (1.0 - e*e)
    / (1.0 - (e*lat.sin()).powi(2)).powf(1.5)
}

/**
Returns the **difference** between the **geographic latitude** and
**geocentric latitude**

# Returns

* ```diff```: Geographic latitude - geocentric latitude *| in radians*

# Arguments

* ```geograph_lat```: Geographic latitude *| in radians*
**/
pub fn geograph_geocen_lat_diff(geograph_lat: f64) -> f64 {
      angle::deg_frm_dms(0, 0, 692.73) * (2.0*geograph_lat).sin()
    - angle::deg_frm_dms(0, 0, 1.16)   * (4.0*geograph_lat).sin()
}

/**
Returns the **equation of time** *| in radians*

# Arguments

* ```JD```: Julian (Ephemeris) day
* ```sun_asc```: Right ascension of the Sun *| in radians*
* ```nut_log```: Nutation correction for longitude *| in radians*
* ```tru_oblq```: True obliquity of the ecliptic *| in radians*
**/
pub fn equation_of_time(JD: f64, sun_asc: f64, nut_long: f64, tru_oblq: f64) -> f64 {
    let t = time::julian_mill(JD);
    let L = angle::limit_to_360(
            280.4664567 +
            t * (360007.6982779 +
            t * (0.030328 +
            t * (1.0/49931.0 -
            t * (1.0/15300.0 +
            t / 2000000.0
            )))));

    (L - 0.0057183 -
     sun_asc.to_degrees() +
     nut_long.to_degrees()*tru_oblq.cos()
    ).to_radians()
}

/**
Returns the **angle between diurnal path** and the **horizon**

# Returns

* ```angl```: Angle between the diurnal path of a celestial body
and the horizon *| in radians*

# Arguments

* ```dec```: Declination of the celestial body *| in radians*
* ```observer_lat```: Observer's geographic latitude *| in radians*
**/
pub fn angl_betwn_diurnal_path_and_hz(dec: f64, observer_lat: f64) -> f64 {
    let B = dec.tan() * observer_lat.tan();
    let C = (1.0 - B*B).sqrt();
    (C * dec.cos()).atan2(observer_lat.tan())
}