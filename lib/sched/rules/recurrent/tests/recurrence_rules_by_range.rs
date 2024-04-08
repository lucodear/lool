use chrono::{Datelike, Local, TimeZone, Timelike};

use crate::sched::rules::{range, ruleset};

#[test]
fn between_10_and_20_seconds() {
    let date = Local.with_ymd_and_hms(2024, 4, 7, 16, 15, 5).unwrap();

    let mut rules = ruleset();
    rules.seconds_rule(range(10, 20, 1));

    let mut next = date;
    let initial_minute = date.minute();

    for i in 0..20 {
        let addend = if i < 11 { i } else { i - 10 - 1 };
        let minute = if i < 11 {
            initial_minute
        } else {
            initial_minute + 1
        };

        next = rules.next_match_from(next).unwrap();
        println!("[{i}]: {:?}", next);

        assert_eq!(
            next,
            Local.with_ymd_and_hms(2024, 4, 7, 16, minute, 10 + addend).unwrap()
        );
    }
}

#[test]
fn each_day_between_9_and_17_at_hour_start() {
    // will start next day because it's already > 17:00:00
    let date = Local.with_ymd_and_hms(2024, 4, 25, 19, 15, 5).unwrap();

    let mut rules = ruleset();
    rules.hours_rule(range(9, 17, 1)).at_minute(0).at_second(0);

    let mut next = date;
    let initial_day = date.day() + 1;

    for i in 0..18 {
        let day = if i < 9 { initial_day } else { initial_day + 1 };
        let addend = if i < 9 { i } else { i - 9 };

        next = rules.next_match_from(next).unwrap();

        println!("[{i}]: {:?}", next);

        assert_eq!(
            next,
            Local.with_ymd_and_hms(2024, 4, day, 9 + addend, 0, 0).unwrap()
        );
    }
}
