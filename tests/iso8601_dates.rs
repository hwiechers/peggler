// Adapted from https://fnordig.de/2015/07/16/omnomnom-parsing-iso8601-dates-using-nom/

#[macro_use]
extern crate peggler;

use peggler::{ParseResult,ParseError};


// Data model

#[derive(Debug,Eq,PartialEq)]
struct Date {
    year: i32,
    month: u32,
    day: u32,
}

#[derive(Debug,Clone,Eq,PartialEq)]
struct Time {
    hour: u32,
    minute: u32,
    second: u32,
    tz_offset: i32,
}

#[derive(Debug,Eq,PartialEq)]
struct DateTime {
    date: Date,
    time: Time,
}

impl Time {
    fn set_tz(&self, tzo: i32) -> Time {
        let mut t = self.clone();
        t.tz_offset = tzo;
        t
    }
}

// Helpers

fn digit(input: &str) -> ParseResult<u32> {
    match input.chars().next() {
        Some(c) => match c.to_digit(10) {
            Some(v) => Ok((v, &input[1..])),
            None    => Err(ParseError),
        },
        None => Err(ParseError)
    }
}

rule!(two_digits:u32 = a:digit b:digit => { 10 * a + b });

rule!(four_digits:u32 = a:digit b:digit c:digit d:digit => {
        1000 * a + 100 * b + 10 * c + d
});

rule!(sign:i32 = ["-"] => { -1 } / ["+"] => { 1 });

// Main Parsing

rule!(positive_year:u32 = four_digits);
rule!(year:i32 = pref:sign? y:positive_year => {
    pref.unwrap_or(1) * (y as i32)
});

rule!(month:u32 = two_digits);
rule!(day:u32 = two_digits);

rule!(date:Date = y:year ["-"] m:month ["-"] d:day => {
    Date { year: y, month: m, day: d }
});

rule!(hour:u32 = two_digits);
rule!(minute:u32 = two_digits);
rule!(second:u32 = two_digits);

rule!(time:Time =
      h:hour [":"] m:minute s:([":"] s:second => { s })? => {
          Time {
              hour: h,
              minute: m,
              second: s.unwrap_or(0),
              tz_offset: 0
          }
});

rule!(timezone_hour:i32 = s:sign h:hour m:([":"]? m:minute)? => {
    (s * (h as i32) * 3600) + (m.unwrap_or(0) * 60) as i32
});

rule!(timezone_utc:i32 = ["Z"] => { 0 });

rule!(timezone:i32 = timezone_utc / timezone_hour);

rule!(datetime:DateTime = d:date ["T"] t:time tzo:timezone? => {
    DateTime { date: d, time: t.set_tz(tzo.unwrap_or(0)) }
});

#[test]
fn test_date() {
    assert_eq!(date("2015-12-31"),
               Ok((Date { year: 2015, month: 12, day: 31 }, "")));
}

#[test]
fn test_time() {
    assert_eq!(time("18:21"),
               Ok((Time { hour: 18,
                          minute: 21,
                          second: 0,
                          tz_offset: 0}, "")));

    assert_eq!(time("18:21:39"),
               Ok((Time { hour: 18,
                          minute: 21,
                          second: 39,
                          tz_offset: 0}, "")));
}

#[test]
fn test_timezone() {
    assert_eq!(timezone("+01"), Ok((3600, "")));
    assert_eq!(timezone("+01:01"), Ok((3660, "")));
    assert_eq!(timezone("+0101"), Ok((3660, "")));
    assert_eq!(timezone("-01"), Ok((-3600, "")));
    assert_eq!(timezone("Z"), Ok((0, "")));
}

#[test]
fn test_datetime() {
    assert_eq!(datetime("1984-01-18T16:55+0200").unwrap().0,
              DateTime {
                date: Date { year: 1984, month: 1, day: 18},
                time: Time { hour: 16, minute: 55, second: 0, tz_offset:7200 }
              });
}
