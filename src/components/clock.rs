use yew::prelude::*;
use chrono::{DateTime, Timelike};
use chrono_tz::Tz;

const HOUR_DEGREE: f32 = 30.0; // 360 / 12 = 30
const MINUTE_DEGREE: f32 = 6.0; // 360 / 60 = 6
const SECOND_DEGREE: f32 = 6.0; // 360 / 60 = 6
const HOUR_MINUTE_FACTOR: f32 = 0.5; // 30 / 60 = 0.5
const MINUTE_SECOND_FACTOR: f32 = 0.1; // 6 / 60 = 0.1

#[derive(Properties, PartialEq)]
pub struct Props {
    pub time: DateTime<Tz>,
}

#[function_component]
pub fn Clock(props: &Props) -> Html {
    let time = props.time.time();

    html! {
        <div class="clock">
            <div style={make_css_rotation(calc_hour_degree(time.hour(), time.minute()))} class="hand main hours" />
            <div style={make_css_rotation(calc_minute_degree(time.minute(), time.second()))} class="hand main minutes" />
            <div style={make_css_rotation(calc_second_degree(time.second()))} class="hand seconds" />
            <div class="point"></div>
            <div class="inner" />
        </div>
    }
}

fn make_css_rotation(degrees: f32) -> String {
    format!("rotate: {}deg;", degrees)
}

fn calc_hour_degree(hours: u32, minutes: u32) -> f32 {
    hours as f32 * HOUR_DEGREE + minutes as f32 * HOUR_MINUTE_FACTOR
}

fn calc_minute_degree(minutes: u32, seconds: u32) -> f32 {
    minutes as f32 * MINUTE_DEGREE + seconds as f32 * MINUTE_SECOND_FACTOR
}

fn calc_second_degree(seconds: u32) -> f32 {
    seconds as f32 * SECOND_DEGREE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_degree() {
        assert_eq!(calc_second_degree(0), 0.0);
        assert_eq!(calc_second_degree(15), 90.0);
        assert_eq!(calc_second_degree(30), 180.0);
        assert_eq!(calc_second_degree(45), 270.0);
        assert_eq!(calc_second_degree(60), 360.0);
    }

    #[test]
    fn test_minute_degree() {
        assert_eq!(calc_minute_degree(0, 0), 0.0);
        assert_eq!(calc_minute_degree(15, 0), 90.0);
        assert_eq!(calc_minute_degree(30, 0), 180.0);
        assert_eq!(calc_minute_degree(45, 0), 270.0);
        assert_eq!(calc_minute_degree(60, 0), 360.0);

        assert_eq!(calc_minute_degree(0, 15), 1.5);
        assert_eq!(calc_minute_degree(15, 15), 91.5);
        assert_eq!(calc_minute_degree(30, 30), 183.0);
        assert_eq!(calc_minute_degree(45, 45), 274.5);
    }

    #[test]
    fn test_hour_degree() {
        assert_eq!(calc_hour_degree(0, 0), 0.0);
        assert_eq!(calc_hour_degree(3, 0), 90.0);
        assert_eq!(calc_hour_degree(6, 0), 180.0);
        assert_eq!(calc_hour_degree(9, 0), 270.0);
        assert_eq!(calc_hour_degree(12, 0), 360.0);

        assert_eq!(calc_hour_degree(0, 15), 7.5);
        assert_eq!(calc_hour_degree(3, 15), 97.5);
        assert_eq!(calc_hour_degree(6, 30), 195.0);
        assert_eq!(calc_hour_degree(9, 45), 292.5);
    }
}