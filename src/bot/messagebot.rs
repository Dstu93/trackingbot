
use bot::commands::traits::MessageCommand;
use std::io::Error;
use std::thread;
use std::thread::JoinHandle;

use teleborg::{Command,Bot,Updater,Dispatcher};
use teleborg::objects::Update;

/// Interface for the Messenger Bots
pub trait MessageBot{

    /// adds commands to the bot
    fn apply(&mut self, cmds: Vec<Box<MessageCommand + Send + Sync>>);

    /// starts the bot in a new Thread and returns the JoinHandle for the freshly spawned thread
    fn start(self) -> Result<JoinHandle<()>,Error>;
}

struct Telegrambot{
    apikey: String,
    commands: Vec<Box<MessageCommand + Send + Sync>>,
}

impl Telegrambot{

    /// creates a new Telegrambot without commands
    pub fn new(apikey: String) -> Telegrambot{
        Telegrambot{commands: Vec::new(),apikey}
    }

}

impl MessageBot for Telegrambot{

    fn apply(&mut self, mut cmds: Vec<Box<MessageCommand + Send + Sync>>) {
        self.commands.append(&mut cmds)
    }

    fn start(self) -> Result<JoinHandle<()>,Error>{

        //Commands an Telegram Wrapper uebergeben
        //Telegram bot bauen

        let mut builder = thread::Builder::new();
        builder.name(String::from("telegrambot_t")).spawn( |  | {
            //TODO start telegram bot here

        })
    }
}

/// Wrapper struct for Telegrambot Implementation
/// wrapps the MessageCommand to the bot specific command trait
pub struct TelegramCmdWrapper{
    cmd: Box<MessageCommand + Send + Sync>,
}

impl TelegramCmdWrapper{
    /// Constructor for TeoelgrambotCommand
    pub fn new(cmd: Box<MessageCommand + Send + Sync>) -> TelegramCmdWrapper{
        TelegramCmdWrapper{cmd}
    }
}

impl Command for TelegramCmdWrapper{
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>){
        let result = self.cmd.exec_cmd_mut(args,None);
        let answer = result.unwrap_or(String::from("Es ist ein Fehler aufgetreten"));
        bot.reply_to_message(&update, &*answer);
    }
}

