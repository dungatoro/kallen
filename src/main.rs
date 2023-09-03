use chrono::{ NaiveDate, Local, Datelike, Duration };
use std::fs::File;

mod args;
use args::{ KallenArgs, Action };
use clap::Parser; 

mod cal;
use cal::{ Calendar, Event, parse_date, parse_time };

mod config;
use config::PATH;

fn main() {

    let mut calendar = match Vec::from_file(PATH) {
        Ok(cal) => cal,
        Err(_) => { let _ = File::create(PATH)
                            .expect("Unable to create {PATH}"); 
                    Vec::new() 
                  }
    };

    if calendar.is_empty() {
        calendar = Vec::init();
    } else {
        calendar.align_left();
    }

    let args = KallenArgs::parse();

    match args.action {
        Action::Add(event) => { 
            let (date, time) = (parse_date(event.date), parse_time(event.time) );
            let new = Event {
                time, 
                desc: event.desc
            };

            calendar.add_event(date, new);
        }

        Action::Del(bad_event) => {
            let date = parse_date(bad_event.date);
            calendar.del_event(date, bad_event.idx); 
        }

        Action::Update(event) => {
            let (date, time) = (parse_date(event.date), parse_time(event.time) );
            let new = Event {
                time, 
                desc: event.desc
            };

            calendar.update_event(date, new, event.idx);
        }

        Action::Today => {
            let t = Local::now();
            let today = NaiveDate::from_ymd_opt(t.year(), t.month(), t.day()).unwrap();

            calendar.print_day(today);
        }

        Action::ThisWeek => {
            let t = Local::now();
            let mut date = NaiveDate::from_ymd_opt(t.year(), t.month(), t.day()).unwrap();

            for _ in 0..7 {
                calendar.print_day(date);
                date += Duration::days(1);
                println!();
            }
        }
    }

    calendar.write(PATH);
}

