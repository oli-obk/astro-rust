extern crate astro;

use astro::*;

#[test]
fn true_anom_and_rad_vec() {
    let t_date = time::Date{year: 1998, month: 8, decimal_day: 5.0, cal_type: time::CalType::Gregorian};
    let t = time::julian_day(&t_date);

    let T_date = time::Date{year: 1998, month: 4, decimal_day: 14.4358, cal_type: time::CalType::Gregorian};
    let T = time::julian_day(&T_date);

    let (tru_anom, rad_vec) = orbit::parabolic::true_anom_and_rad_vec(t, T, 1.487469);
    assert_eq!(util::round_upto_digits(tru_anom.to_degrees(), 5), 66.78862);
    assert_eq!(util::round_upto_digits(rad_vec, 6), 2.133911);
}

#[test]
fn passage_through_nodes() {
    let T = time::julian_day(&time::Date{
        year: 1989,
        month: 8,
        decimal_day: 20.291,
        cal_type: time::CalType::Gregorian
    });
    let w = 154.9103_f64.to_radians();
    let q = 1.324502;

    let (ascen, r_a) = orbit::parabolic::passage_through_node(w, q, T, &orbit::Node::Ascend);
    assert_eq!(util::round_upto_digits((T - ascen), 2), 4354.65);
    assert_eq!(util::round_upto_digits(r_a, 2), 28.07);

    let (descend, r_b) = orbit::parabolic::passage_through_node(w, q, T, &orbit::Node::Descend);
    assert_eq!(util::round_upto_digits((T - descend), 4), -28.3454);
    assert_eq!(util::round_upto_digits(r_b, 4), 1.3901);
}
