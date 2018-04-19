
use bot::commands::services::TrackingStateService;
use bot::commands::dataobjects::TrackingState;
use bot::commands::command_implementations::TrackingStateCommand;
use bot::commands::traits::{CommandError,MessageCommand};
use tests::mocks::TrackingStateServiceMock;

#[test]
pub fn track_cmd_test(){
    let service = Box::new(TrackingStateServiceMock::new());
    let cmd = TrackingStateCommand::new(service);
    let chat = None;

    let invalid_arg = None;
    let result = cmd.exec_cmd(invalid_arg,chat);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(),CommandError::NoArguments);

    let empty_vec = Some(Vec::new());
    let result = cmd.exec_cmd(empty_vec,chat);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(),CommandError::NoArguments);

    let nan = Some(vec!["abcdefg"]);
    let result = cmd.exec_cmd(nan,chat);
    assert_eq!(result.unwrap_err(),CommandError::IllegalArguments);

    // keine Zeit für den Test
    let valid_1 = Some(vec!["0000"]);
    let result = cmd.exec_cmd(valid_1,chat);
    let state = TrackingStateServiceMock::new().state("0000").unwrap().unwrap();
    let expected_answer = format!("Trackingcode: {}\n,Lieferadresse: {}\n,Entgegen genommen von: {}\n,Lieferstatus: {}",
                                  state.access_key(),state.delivery_address(), state.delivered_to(),state.state());

    assert_eq!(result.unwrap(),expected_answer);
}