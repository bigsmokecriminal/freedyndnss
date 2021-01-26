extern crate config;
extern crate serde;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

mod settings;
mod dyndnss;

use settings::{Settings};
use dyndnss::make_updates;
use std::env;




fn main() {
    let args: Vec<String> = env::args().collect();
    let moc_home_dir = String::from(&args[1]);
    let settings = Settings::from_file(&moc_home_dir);
    let config : Settings;
    match settings{
        Ok(_) => {
            config = settings.unwrap();
            make_updates(&config.domains, &config.url.url);
        },
        Err(_) => println!("FreeDynDnss: Cannot find config file!")
    }
}
