extern crate chrono;
extern crate lettre;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use std::error::Error;
use std::thread::JoinHandle;

use chrono::{Datelike,DateTime,Duration,Local,Timelike,Weekday};
use lettre::email::EmailBuilder;
use lettre::transport::EmailTransport;
use lettre::transport::smtp::SmtpTransportBuilder;

mod jobs;
mod secrets;

use jobs::Job;

#[allow(dead_code)]
enum Month {
    Jan =  1,
    Feb =  2,
    Mar =  3,
    Apr =  4,
    May =  5,
    Jun =  6,
    Jul =  7,
    Aug =  8,
    Sep =  9,
    Oct = 10,
    Nov = 11,
    Dec = 12,
}

fn is_purple_daze(dt: DateTime<Local>) -> bool {
    let is_purple_wed = (dt.weekday() == Weekday::Wed) & (3 < dt.day()) & (dt.day() < 11);
    let is_purple_thu = (dt.weekday() == Weekday::Thu) & (21 == dt.day()) & (dt.month() == Month::Nov as u32);
    let is_purple_fri = (dt.weekday() == Weekday::Fri) & (dt.day() < 6) | (dt.day() > 12);
    is_purple_wed | is_purple_thu | is_purple_fri
}

fn is_purple_daze_now() -> bool {
    is_purple_daze(Local::now())
}

fn email_if_purple_daze() -> Result<(), Box<Error>> {
    let now = Local::now();
    if true | (now.hour() == 17) & is_purple_daze(now + Duration::days(1)) {
        println!("Is purpledaze tomorrow");
        let email = EmailBuilder::new()
                            .to(secrets::TEST_EMAIL)
                            .from(secrets::MY_EMAIL)
                            .body("test")
                            .subject("Test")
                            .build()?;

        let mut mailer = SmtpTransportBuilder::localhost()?.build();
        mailer.send(email)?;
        println!("Purple Daze reminder sent");
    } else {
        println!("Is not purpledaze tomorrow");
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct PurpleSubs {
    subs: Vec<String>,
    last_id: String
}

pub fn get_purple_subs() {
    let purple_file = File::open("subscribers.txt").unwrap();
    let mut buf_reader = BufReader::new(purple_file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let decoded: PurpleSubs = serde_json::from_str(&contents).unwrap();
    println!("{}", serde_json::to_string_pretty(&decoded).unwrap());
}

pub fn run_purple_mailer(wait_time: u64) -> JoinHandle<()> {
    let sj = jobs::StandardJob::new(wait_time, email_if_purple_daze);
    sj.run()
}

