//! Time for astronomy

use angle;
use ecliptic;
use nutation;

/// Represents a calendar type
pub enum CalType {
    /// Gregorian calendar
    Gregorian,
    /// Julian calendar
    Julian
}

/// Represents a date with year, month, decimal day and calendar type
pub struct Date {
    /// Year
    pub year: i32,
    /// Month
    ///
    /// range: *1 - 12*
    pub month: u8,
    /// Decimal day
    pub decimal_day: f64,
    /// Calendar type
    pub cal_type: CalType,
}

/// Represents a day of a month with hours, minutes and seconds
pub struct DayOfMonth {
    /// Day of month
    ///
    /// range: *1 - 31*
    pub day: u8,
    /// Hour of day
    ///
    /// range: *0 - 60*
    pub hr: u8,
    /// Minute of hour
    ///
    /// range: *0 - 60*
    pub min: u8,
    /// Second of minute
    ///
    /// range: *0.0 - 60.0*
    pub sec: f64,
    /// Time zone offset *| in decimal days*
    pub time_zone: f64,
}

/**
Returns decimal day for a `DayOfMonth`

* `day_of_month`: A `DayOfMonth` struct
**/
pub fn decimal_day(day: &DayOfMonth) -> f64 {
      (day.day as f64)
    + (day.hr as f64)/24.0
    + (day.min as f64)/60.0
    +  day.sec/60.0
    -  day.time_zone/24.0
}

/**
Returns decimal year for a `Date`

* `date`: A `Date` struct
**/
pub fn decimal_year(date: &Date) -> f64 {
    let mut y = 0;
    let mut days = 365.0;

    if date.month > 1 { y += 31; }
    if date.month > 2 {
        y += 28;
        if is_leap_year(date.year, &date.cal_type) {
            y += 1;
            days += 1.0;
        }
    }
    if date.month > 3  { y += 31; }
    if date.month > 4  { y += 30; }
    if date.month > 5  { y += 31; }
    if date.month > 6  { y += 30; }
    if date.month > 7  { y += 31; }
    if date.month > 8  { y += 31; }
    if date.month > 9  { y += 30; }
    if date.month > 10 { y += 31; }
    if date.month > 11 { y += 30; }

    (date.year as f64) + ((y as f64) + date.decimal_day)/days
}

/**
Checks if a year is a leap year

# Arguments

* `year`    : Year
* `cal_type`: `CalType` enum
**/
pub fn is_leap_year(year: i32, cal_type: &CalType) -> (bool) {
    match cal_type {
        &CalType::Julian    => year % 4 == 0,
        &CalType::Gregorian => {
            if year % 100 == 0 { year % 400 == 0 }
            else               { year % 4 == 0   }
        },
    };

    false
}

/**
Returns Julian century for a Julian day

# Arguments

* `JD`: Julian (Ephemeris) day
**/
pub fn julian_cent(JD: f64) -> f64 {
    (JD - 2451545.0) / 36525.0
}

/**
Returns Julian millennium for a Julian day

# Arguments

* `JD`: Julian (Ephemeris) day
**/
pub fn julian_mill(JD: f64) -> f64 {
    (JD - 2451545.0) / 365250.0
}

/**
Returns Julian day for a `Date`

# Arguments

`date`: A `Date` struct
**/
pub fn julian_day(date: &Date) -> f64 {
    let mut y = date.year;
    let mut m = date.month;
    if m == 1 || m == 2 {
        y -= 1;
        m += 12;
    }

    let a = ((y as f64) / 100.0).floor();
    let b = match date.cal_type {
        CalType::Gregorian => 2.0 - a + (a/4.0).floor(),
        CalType::Julian    => 0.0,
    };

      (365.25  * ((y as f64) + 4716.0)).floor()
    + (30.6001 * ((m as f64) + 1.0   )).floor()
    + (date.decimal_day as f64)
    + (b as f64)
    - 1524.5
}

/**
Returns the Julian Ephemeris day

# Arguments

* `JD`     : Julian day
* `delta_t`: Delta T
**/
pub fn julian_emph_day(JD: f64, delta_t: f64) -> f64 {
    delta_t/86400.0 + JD
}

/**
Returns a year, month and decimal day equivalent to a given Julian day

# Returns

`(year, month, decimal_day)`

* `year`       : Year
* `month`      : Month
* `decimal_day`: Decimal day

# Arguments

`JD`: Julian Day. **Can't be a negative value.**
**/
pub fn date_frm_julian_day(mut JD: f64) -> (i16, u8, f64) {
    if JD < 0.0 {
        panic!("A negative value for JD was passed to time::date_frm_julian_day()");
    }

    JD += 0.5;
    let Z = JD as i64;
    let F = JD - (Z as f64);
    let mut A;

    if Z < 2299161 {
        A = Z;
    }
    else {
        let alpha = (((Z as f64) - 1867216.25) / 36524.25).floor() as i64;
        A = Z + 1 + alpha - (((alpha as f64)/4.0).floor() as i64);
    }

    let B = A + 1524;
    let C = (((B as f64) - 122.1) / 365.25).floor()  as i64;
    let D = (365.25 * (C as f64)).floor() as i64;
    let E = (((B - D) as f64)/30.6001).floor() as i64;

    let day = ((B - D) as f64) - (30.6001 * (E as f64)).floor() + F;

    let month =      if E < 14             { E - 1  }
                else if E == 14 || E == 15 { E - 13 }
                else {
                    panic!("Internal error in time::date_frm_julian_day()");
                    0
                };

    let year =      if month > 2                { C - 4716 }
               else if month == 1 || month == 2 { C - 4715 }
               else {
                   panic!("Internal error in time::date_frm_julian_day()");
                   0
               };

    (year as i16, month as u8, day)
}

/**
Returns apparent sidereal time from the mean sidereal time

# Returns

* `apprnt_sidr`: Apparent sidereal time *| in radians*

# Arguments

* `mn_sidr`    : Mean sidereal time *| in radians*
* `nut_in_long`: Nutatation in longitude *| in radians*
* `true_oblq`  : True obliquity of the ecliptic *| in radians*
**/
pub fn apprnt_sidr(mn_sidr: f64, nut_in_long: f64, true_oblq: f64) -> f64 {
    mn_sidr + nut_in_long*true_oblq.cos()
}

/**
Returns apparent sidereal time for a Julian day

This functions uses internally J. Laskar's formula for
computing the obliquity of the ecliptic.

# Returns

* `apprnt_sidr`: Apparent sidereal time *| in radians*

# Arguments

* `$JD`: Julian day
**/
#[macro_export]
macro_rules! apprnt_sidr {
    ($JD: expr) => {{
        let (nut_in_long, nut_in_oblq) = astro::nutation::nutation($JD);
        let eclip_oblq = astro::ecliptic::mn_oblq_laskar($JD);
        astro::time::apprnt_sidr(astro::time::mn_sidr($JD), nut_in_long, eclip_oblq + nut_in_oblq)
    }};
}

/**
Returns mean sidereal time for a Julian day

# Returns

* `mn_sidr`: Mean sidereal time *| in radians*

# Arguments

* `JD`: Julian day
**/
pub fn mn_sidr(JD: f64) -> f64 {
    let JC = julian_cent(JD);
    let angle = angle::limit_to_360(  280.46061837
                                    + 360.98564736629 * (JD - 2451545.0)
                                    + JC*JC * (0.000387933 - JC/38710000.0)
                                   );

    angle.to_radians()
}

/**
Returns an approximate value of ΔT for a given year and month

This function approximates ΔT from polynomial expressions using a
method different from that given in the *Meeus* book. The method
used is given [here](http://eclipse.gsfc.nasa.gov/SEcat5/deltatpoly.html);
it covers a far wider time range, and is more accurate.

# Arguments

* `year` : Year
* `month`: Month *range: 1 - 12*
**/
pub fn approx_delta_t(year: i32, month: u8) -> f64 {
    let y = (year as f64) + ((month as f64) - 0.5) / 12.0;

    if y < -500.0 {
        let u = (y-1820.0) / 100.0;
        return -20.0 + 32.0*u*u;
    }
    else if y < 500.0 {
        let u = y / 100.0;
        return 10583.6 -
               u * (1014.41 +
               u * (33.78311 +
               u * (5.952053 -
		       u * (0.1798452 -
               u * (0.022174192 -
                    u * 0.0090316521)
                ))));
    }
    else if y < 1600.0 {
        let u = (y-1000.0) / 100.0;
        return 1574.2 -
               u * (556.01 -
               u * (71.23472 +
               u * (0.319781 -
		       u * (0.8503463 +
               u * (0.005050998 -
                    u * 0.0083572073)
                   ))));
    }
    else if y < 1700.0 {
        let u = y - 1600.0;
        return 120.0 -
               u * (0.9808 +
               u * (0.01532 -
                    u / 7129.0
                   ));
    }
    else if y < 1800.0 {
        let u = y - 1700.0;
        return 8.83 +
               u * (0.1603 -
               u * (0.0059285 -
               u * (0.00013336 +
                    u / 1174000.0
                   )));
    }
    else if y < 1860.0 {
        let u = y - 1800.0;
        return 13.72 -
               u * (0.332447 -
               u * (0.0068612 +
               u * (0.0041116 -
               u * (0.00037436 -
               u * (0.0000121272 +
               u * (0.0000001699 -
                    u * 0.000000000875
                   ))))));
    }
    else if y < 1900.0 {
        let u = y - 1860.0;
        return 7.62 +
               u * (0.5737 -
               u * (0.251754 -
               u * (0.01680668 -
               u * (0.0004473624 +
                    u / 233174.0
                   ))));
    }
    else if y < 1920.0 {
        let u = y - 1900.0;
        return -2.79 +
               u * (1.494119 -
               u * (0.0598939 -
               u * (0.0061966 +
                    u * 0.000197
                   )));
    }
    else if y < 1941.0 {
        let u = y - 1920.0;
        return 21.20 +
               u * (0.84493 -
               u * (0.076100 -
                    u * 0.0020936
                   ));
    }
    else if y < 1961.0 {
        let u = y - 1950.0;
        return 29.07 +
               u * (0.407 -
               u * ((1.0 / 233.0) -
                    u / 2547.0
                   ));
    }
    else if y < 1986.0 {
        let u = y - 1975.0;
        return 45.45 +
               u * (1.067 -
               u * ((1.0 / 260.0) +
                    u / 718.0)
                   );
    }
    else if y < 2005.0 {
        let u = y - 2000.0;
        return 63.86 +
               u * (0.3345 -
               u * (0.060374 -
               u * (0.0017275 +
               u * (0.000651814 +
                    u * 0.00002373599)
                   )));
    }
    else if y < 2050.0 {
        let u = y - 2000.0;
        return 62.92 +
               u * (0.32217 +
                    u * 0.005589);
    }
    else if y <= 2150.0 {
        let u = (y - 1820.0) / 100.0;
        return -20.0 + 32.0*u*u - 0.5628*(2150.0 - y);
    }
    else if y > 2150.0 {
        let u = (y - 1820.0) / 100.0;
        return -20.0 + 32.0*u*u;
    }

    0.0
}
