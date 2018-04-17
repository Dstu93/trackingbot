
use bot::commands::traits::MessageCommand;
use std::io::Error;
use std::thread::JoinHandle;

pub trait MessageBot{
    fn apply(&mut self, cmds: Vec<Box<MessageCommand + Send + Sync>>);
    fn start(&mut self) -> Result<JoinHandle<()>,Error>;
}