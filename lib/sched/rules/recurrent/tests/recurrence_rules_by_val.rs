use chrono::{Datelike, Local, TimeZone, Timelike, Weekday};

use crate::sched::rules::ruleset;

#[test]
fn at_second_1_of_each_minute() {
    // we have passed the second 1 of the minute
    // so it should go to the next minute
    let date = Local.with_ymd_and_hms(2024, 4, 7, 16, 15, 05).unwrap();

    let mut rules = ruleset();
    rules.at_second(1);

    let mut next = date.clone();
    let initial_minute = date.minute();

    for i in 0..10 {
        next = rules.next_match_from(next).unwrap();
        println!("next: {:?}", next);
        // should match 16:16:01, 16:17:01, 16:18:01, ...
        assert_eq!(
            next,
            Local.with_ymd_and_hms(2024, 4, 7, 16, initial_minute + i + 1, 01).unwrap()
        );
    }
}

#[test]
fn at_hour_1_of_each_day() {
    // we have passed the hour 1 of the day
    // so it should go to the next day
    let date = Local.with_ymd_and_hms(2024, 4, 25, 16, 15, 05).unwrap();

    let mut rules = ruleset();
    rules.at_time(1, 0, 0);

    let mut next = date.clone();
    let mut initial_day = date.day() as i32;
    let mut initial_month = date.month();

    for i in 0..10 {
        next = rules.next_match_from(next).unwrap();
        println!("next: {:?}", next);

        if initial_day + i + 1 == 31 {
            initial_day = -i;
            initial_month += 1;
        }

        // should match 2024-04-08 01:00:00, 2024-04-09 01:00:00, 2024-04-10 01:00:00, ...
        assert_eq!(
            next,
            Local
                .with_ymd_and_hms(
                    2024,
                    initial_month,
                    (initial_day + i + 1) as u32,
                    01,
                    00,
                    00
                )
                .unwrap()
        );
    }
}

#[test]
fn each_1st_of_month() {
    // we have passed the 1st of the month
    // so it should go to the next month
    let date = Local.with_ymd_and_hms(2024, 5, 7, 16, 15, 05).unwrap();

    let mut rules = ruleset();
    rules.on_day(1).at_time(0, 0, 0);

    let mut next = date.clone();
    let mut initial_month = date.month() as i32;
    let mut initial_year = date.year();

    for i in 0..10 {
        next = rules.next_match_from(next).unwrap();
        println!("next: {:?}", next);

        if initial_month + i + 1 == 13 {
            initial_month = -i;
            initial_year += 1;
        }

        // should match 2024-05-01 00:00:00, 2024-06-01 00:00:00, 2024-07-01 00:00:00, ...
        assert_eq!(
            next,
            Local
                .with_ymd_and_hms(initial_year, (initial_month + i + 1) as u32, 01, 00, 00, 00)
                .unwrap()
        );
    }
}

#[test]
fn each_wednesday() {
    // we have passed the Wednesday
    // so it should go to the next Wednesday
    let date = Local.with_ymd_and_hms(2024, 4, 7, 16, 15, 05).unwrap();
    let mut next_wednesday = Local.with_ymd_and_hms(2024, 4, 10, 0, 0, 0).unwrap();

    let mut rules = ruleset();
    rules.on_dow(Weekday::Wed).at_time(0, 0, 0);

    let mut next = date.clone();

    for _ in 0..10 {
        next = rules.next_match_from(next).unwrap();
        println!("next: {:?}", next);

        assert_eq!(next, next_wednesday);
        next_wednesday = next_wednesday + chrono::Duration::days(7);
    }
}

#[test]
fn from_31th_may_schedule_first_of_each_june() {
    // we have passed the 31th of May
    // so it should go to the 1st of June
    let date = Local.with_ymd_and_hms(2024, 5, 31, 16, 15, 05).unwrap();

    let mut rules = ruleset();
    rules.in_month(6).on_day(1).at_time(0, 0, 0);

    let mut next = date.clone();

    let initial_year = date.year();
    let initial_month = date.month();

    for i in 0..10 {
        next = rules.next_match_from(next).unwrap();
        println!("next: {:?}", next);

        // should match 2024-06-01 00:00:00, 2024-07-01 00:00:00, 2024-08-01 00:00:00, ...
        assert_eq!(
            next,
            Local.with_ymd_and_hms(initial_year + i, initial_month + 1, 01, 00, 00, 00).unwrap()
        );
    }
}

#[test]
fn from_1st_may_schedule_first_of_each_june() {
    let date = Local.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();

    let mut rules = ruleset();
    rules.in_month(6).on_day(1).at_time(0, 0, 0);

    let mut next = date.clone();
    let initial_year = date.year();

    for i in 0..10 {
        next = rules.next_match_from(next).unwrap();
        println!("next: {:?}", next);

        assert_eq!(
            next,
            Local.with_ymd_and_hms(initial_year + i + 1, 06, 01, 00, 00, 00).unwrap()
        );
    }
}

#[test]
fn if_full_date_set_should_return_only_one_match_then_null() {
    let date = Local.with_ymd_and_hms(2024, 5, 14, 19, 44, 15).unwrap();

    let mut rules = ruleset();
    rules.on_datetime(2024, 6, 1, 0, 0, 0);

    let next = rules.next_match_from(date).unwrap();
    assert_eq!(next, Local.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap());

    let next = rules.next_match_from(next);
    assert!(next.is_none());
}
