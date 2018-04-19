
use bot::commands::services::TrackingStateService;
use bot::commands::traits::{CommandError,CommandType,MessageCommand,Chat};
use std::str::FromStr;

pub struct TrackingStateCommand{
    service: Box<TrackingStateService + Send + Sync>,
}

impl TrackingStateCommand{

    pub fn new(service: Box<TrackingStateService + Send + Sync>) -> TrackingStateCommand{
        TrackingStateCommand{service}
    }

}

impl MessageCommand for TrackingStateCommand{

    fn exec_cmd(&self, args: Option<Vec<&str>>, chat: Option<&Chat>) -> Result<String,CommandError>{

        if args.is_none(){
            return Err(CommandError::NoArguments);
        }
        let arguments = args.unwrap();
        if arguments.len()  < 1{
            return Err(CommandError::NoArguments);
        }
        if arguments.len() > 1 {
             return Err(CommandError::IllegalArguments);
        }

        //vorvalidierung
        let tracking_code = arguments.get(0).unwrap();
        let parse_validation = u64::from_str(tracking_code);
        if parse_validation.is_err() {
           return Err(CommandError::IllegalArguments);
        }

        let result = self.service.state(tracking_code);
        if result.is_err(){
            println!("Ein Fehler bei der Datenbank abfrage ist aufgetrteten: {:#?}", result.unwrap_err());
            return Err(CommandError::IOError);
        }
        let state = result.unwrap();
        if state.is_none(){
            return Ok(String::from("Kein Status gefunden"));
        }
        let state = state.unwrap();
        let answer = format!("Trackingcode: {},\nLieferadresse: {},\nEntgegen genommen von: {},\nLieferstatus: {}",
                                      state.access_key(),state.delivery_address(), state.delivered_to(),state.state());
        Ok(answer)
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

