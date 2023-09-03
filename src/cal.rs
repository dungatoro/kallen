use chrono::prelude::*;
use chrono::Duration;
use serde::{Serialize, Deserialize};

use std::error::Error;
use std::io::Read;

use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub time: Option<NaiveTime>,
    pub desc: String,
}

#[derive(Serialize, Deserialize)]
pub struct Day {
    pub date: NaiveDate,
    pub plan: Vec<Event>
}

impl Day {
    pub fn sort_plan(&mut self) {
        self.plan
            .sort_by(|x, y| {
            let time1 = x.time;
            let time2 = y.time;

            match (time1, time2) {
                (Some(t1), Some(t2)) => t1.cmp(&t2),
                (None, None) => std::cmp::Ordering::Equal,
                (None, _) => std::cmp::Ordering::Less,
                (_, None) => std::cmp::Ordering::Greater,
            }
        });
    }
}

pub trait Calendar {
    fn init() -> Vec<Day>;
    fn align_left(&mut self); // keep usr in 1st yr of 2 yr calendar
    fn from_file(path: &str) -> Result<Self, Box<dyn Error>> where Self: Sized;
    fn write(&self, path: &str);

    fn find_date(&self, date: NaiveDate) -> Option<usize>;

    fn add_event(&mut self, date: NaiveDate, new_event: Event);
    fn update_event(&mut self, date: NaiveDate, new_event: Event, idx: usize);
    fn del_event(&mut self, date: NaiveDate, idx: usize);
}

impl Calendar for Vec<Day> {
    fn init() -> Vec<Day> {
        let mut calendar = Vec::new();
    
        let year = Local::now().year();

        let mut date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap(); 
        let end = NaiveDate::from_ymd_opt(year+1, 12, 31).unwrap(); 
    
        while date <= end {
            calendar.push(Day { date, plan: Vec::new() } ); 
            date += Duration::days(1); 
        }
    
        calendar
    }

    fn align_left(&mut self) {
        let year = Local::now().year();

        if self[0].date.year() < year {
            self.drain(0..365);
        }

        let mut date = NaiveDate::from_ymd_opt(year+1, 1, 1).unwrap(); 
        let end = NaiveDate::from_ymd_opt(year+1, 12, 31).unwrap(); 
    
        while date <= end {
            self.push(Day { date, plan: Vec::new() } ); 
            date += Duration::days(1); 
        }
    }

    fn from_file(path: &str) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let calendar: Vec<Day> = serde_json::from_str(&contents)?;
        Ok(calendar)
    }

    fn write(&self, path: &str) {
        let json_str = serde_json::to_string(&self).expect("Unable to serialize");
        let mut file = File::create(path).expect("Unable to find file for writing");
        file.write_all(json_str.as_bytes()).expect("Unable to write to file");
    }

    fn find_date(&self, date: NaiveDate) -> Option<usize> {
        self.iter()
            .position(|day| day.date == date)
    }

    fn add_event(&mut self, date: NaiveDate, new_event: Event) {
        match self.find_date(date) {
            None => { println!("Invalid date"); },
            Some(p) => { 
                self[p].plan.push(new_event); 
                self[p].sort_plan();
            }
        }
    }

    fn update_event(&mut self, date: NaiveDate, new_event: Event, idx: usize) {
        match self.find_date(date) {
            None => { println!("Invalid date"); },
            Some(p) => { 
                self[p].plan[idx] = new_event;
                self[p].sort_plan();
            }
        }
    }

    fn del_event(&mut self, date: NaiveDate, idx: usize) {
        match self.find_date(date) {
            None => { println!("Invalid date"); },
            Some(p) => { 
                self[p].plan.remove(idx);
            }
        }
    }
}

pub fn parse_date(date: String) -> NaiveDate {
    if date.is_empty() {
        let t = Local::now();
        NaiveDate::from_ymd_opt(t.year(), t.month(), t.day()).unwrap()
    } else {
        NaiveDate::parse_from_str(date.as_str(), "%d/%m/%Y").expect("invalid time format: use hr:mn")
    }
}

pub fn parse_time(time: String) -> Option<NaiveTime> {
    if time.is_empty() {
        None
    } else {
        Some( NaiveTime::parse_from_str(time.as_str(), "%H:%M").expect("invalid time format: use hr:mn") )
    }
}
