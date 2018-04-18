
use std::error::Error;
use std::fmt;

/// Abstraction of an Command for this bot
pub trait MessageCommand{
    /// executes the command in immutable way
    /// the arguments of this command can be accessed with the args variable.
    fn exec_cmd(&self, args: Option<Vec<&str>>, chat: Option<&Chat>) -> Result<String,CommandError>;

    /// Same like exec_cmd but in an mutable way
    fn exec_cmd_mut(&mut self, args: Option<Vec<&str>>,chat: Option<&Chat>) -> Result<String,CommandError>;

    /// shows the type of this Command
    fn cmd_type(&self) -> &CommandType;

    /// returns the name for calling this command
    fn name(&self) -> &str;

    /// flag for commands which can only be executed with a mutable state
    fn needs_mut(&self) -> bool;
}

/// Enum Type to present the type of Command
/// SingleCommand is an Command which just get the answer to the command
/// ChatCommand can react on previous messages with the user
#[derive(Debug,Clone,Hash,Eq, PartialEq,)]
pub enum CommandType{
    SingleCommand,
    ChatCommand,
}

/// placeholder struct for presenting the a Messenger Chat
#[derive(Eq, PartialEq,Hash,Clone,Debug)]
pub struct Chat{
    history: Vec<Message>,
}

/// placeholder struct for presenting an Message with a user
#[derive(Eq, PartialEq,Hash,Clone,Debug)]
pub struct Message{
}

#[derive(Eq, PartialEq,Hash, Clone,Debug)]
pub enum CommandError{
    InternalCommandError,
    IllegalArguments,
    NoArguments,
    IOError,
}

impl Error for CommandError{
    fn description(&self) -> &str{
        match self {
            &CommandError::InternalCommandError => "Internal Server Error",
            &CommandError::IllegalArguments => "illegal arguments",
            &CommandError::NoArguments => "no arguments was passed",
            &CommandError::IOError => "IOError",
        }
    }
}

impl fmt::Display for CommandError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommandError::InternalCommandError => write!(f, "InternalCommandError"),
            CommandError::IllegalArguments => write!(f, "IllegalArguments"),
            CommandError::NoArguments=> write!(f, "NoArguments"),
            CommandError::IOError => write!(f,"IOerror"),
        }
    }
}

//TODO implement From<io::Error> for CommandError