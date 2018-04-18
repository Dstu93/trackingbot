
extern crate postgres;
extern crate serde;
extern crate serde_json;
extern crate teleborg;

#[macro_use]
extern crate serde_derive;

mod bot;
mod tests;

use bot::configuration::loader;
use bot::configuration::config::ApplicationConfig;

fn main() {
    let config = loader::applicationconfig();

    //mit BotBuilder den MessangerBot bauen
    //bot starten
    //warten auf thread join

    println!("Hello, world!");
}
