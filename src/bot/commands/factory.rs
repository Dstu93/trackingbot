
use bot::configuration::config::ApplicationConfig;
use bot::commands::traits::{MessageCommand};
use bot::commands::command_implementations::TrackingStateCommand;
use bot::commands::services::*;

pub struct MessageCommandFactory;

impl MessageCommandFactory{

    pub fn build(config: &ApplicationConfig) -> Vec<Box<MessageCommand + Send + Sync>>{
        let mut commands: Vec<Box<MessageCommand + Send + Sync>> = Vec::new();
        let tracking_service = Box::new(TrackingStateServiceImpl::new(config.db_conf()));
        let tracking_cmd = Box::new(TrackingStateCommand::new(tracking_service));
        commands.push(tracking_cmd);

        commands
    }

}