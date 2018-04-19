
extern crate postgres;
extern crate serde;
extern crate serde_json;
extern crate teleborg;
#[macro_use]
extern crate mysql;

#[macro_use]
extern crate serde_derive;

mod bot;

#[cfg(test)]
mod tests;

use bot::configuration::loader as configloader;
use bot::messagebot::{BotType,BotBuilder,MessageBot};
use bot::configuration::config::ApplicationConfig;
use bot::commands::factory::MessageCommandFactory;

fn main() {
   let config = configloader::applicationconfig().expect("could not load config");

    //mit BotBuilder den MessangerBot bauen
    //bot starten
    //warten auf thread join
    let mut bot = BotBuilder::build(&config,BotType::Telegram);
    let cmds = MessageCommandFactory::build(&config);

    bot.apply(cmds);

    let mut handle = bot.start().expect("Bot konnte nicht gestartet werden");

    handle.join(); //stopt den Main Thread bis der Thread des Bots terminiert und die Threads gejoint werden koennen.
}
