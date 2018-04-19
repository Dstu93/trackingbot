
use bot::commands::services::TrackingStateService;
use bot::commands::traits::{CommandError,CommandType,MessageCommand,Chat};

pub struct TrackingStateCommand{
    service: Box<TrackingStateService>,
}

impl TrackingStateCommand{

    pub fn new(service: Box<TrackingStateService>) -> TrackingStateCommand{
        TrackingStateCommand{service}
    }

}

impl MessageCommand for TrackingStateCommand{

    fn exec_cmd(&self, args: Option<Vec<&str>>, chat: Option<&Chat>) -> Result<String,CommandError>{
        //TODO
        // parsen der str zu u64
        //code an service weiter geben
        //TrackingState zu String formatieren
        Err(CommandError::IllegalArguments)
    }

    fn exec_cmd_mut(&mut self, args: Option<Vec<&str>>,chat: Option<&Chat>) -> Result<String,CommandError>{
        self.exec_cmd(args,chat)
    }

    fn cmd_type(&self) -> &CommandType{
        &CommandType::SingleCommand
    }

    fn name(&self) -> &str{
        "track"
    }

    fn needs_mut(&self) -> bool{
        false
    }
}

