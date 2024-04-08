use chrono::{Datelike, Local, TimeZone, Utc};

use crate::sched::rules::{many, range, ruleset};

#[test]
fn every_day_at_12_and_15() {
    let date = Local.with_ymd_and_hms(2024, 4, 7, 13, 15, 05).unwrap();

    let mut rules = ruleset();
    rules.hours_rule(many(vec![12, 15])).at_minute(0).at_second(0);

    let next = rules.next_match_from(date).unwrap();
    assert_eq!(next, Local.with_ymd_and_hms(2024, 4, 7, 15, 0, 0).unwrap());

    let next = rules.next_match_from(next).unwrap();
    assert_eq!(next, Local.with_ymd_and_hms(2024, 4, 8, 12, 0, 0).unwrap());

    let next = rules.next_match_from(next).unwrap();
    assert_eq!(next, Local.with_ymd_and_hms(2024, 4, 8, 15, 0, 0).unwrap());

    let next = rules.next_match_from(next).unwrap();
    assert_eq!(next, Local.with_ymd_and_hms(2024, 4, 9, 12, 0, 0).unwrap());

    let next = rules.next_match_from(next).unwrap();
    assert_eq!(next, Local.with_ymd_and_hms(2024, 4, 9, 15, 0, 0).unwrap());
}

#[test]
fn each_day_between_9_and_17_at_hour_start() {
    // will start next day because it's already > 17:00:00
    let date = Local.with_ymd_and_hms(2024, 4, 25, 19, 15, 05).unwrap();

    let mut rules = ruleset();
    rules.hours_rule(range(9, 17, 1)).at_minute(0).at_second(0);

    let mut next = date.clone();
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
