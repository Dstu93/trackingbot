
use bot::commands::services::TrackingStateService;
use bot::commands::traits::{CommandError,CommandType,MessageCommand,Chat};

#[derive(Debug)]
pub struct TrackingStateCommand{
    service: Box<TrackingStateService>,
}

impl TrackingStateCommand{

    pub fn new(service: Box<TrackingStateCommand>) -> TrackingStateCommand{
        TrackingStateCommand{service}
    }

}

impl MessageCommand for TrackingStateCommand{

    fn exec_cmd(&self, args: Option<Vec<&str>>, chat: Option<&Chat>) -> Result<String,CommandError>{
        //TODO
    }

    fn exec_cmd_mut(&mut self, args: Option<Vec<&str>>,chat: Option<&Chat>) -> Result<String,CommandError>{
        self.exec_cmd(args,chat)
    }

    fn cmd_type(&self) -> &CommandType{
        &CommandType::SingleCommand
    }

    /// returns the name for calling this command
    fn name(&self) -> &str{
        "track"
    }

    /// flag for commands which can only be executed with a mutable state
    fn needs_mut(&self) -> bool{
        false
    }
}

