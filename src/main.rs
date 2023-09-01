use chrono::prelude::*;
use chrono::Duration;

struct Event {
    time: Option<NaiveTime>,
    desc: String,
}

struct Day {
    date: NaiveDate,
    plan: Vec<Event>
}

trait Calendar {
    fn init() -> Vec<Day>;

    fn align_left(&mut self);
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

        // create the next year so that we remain in the left half of the 2 year calendar
        let mut date = NaiveDate::from_ymd_opt(year+1, 1, 1).unwrap(); 
        let end = NaiveDate::from_ymd_opt(year+1, 12, 31).unwrap(); 
    
        while date <= end {
            self.push(Day { date, plan: Vec::new() } ); 
            date += Duration::days(1); 
        }

    }
}

fn main() {

    // read from JSON using serde

}

