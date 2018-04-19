//
//#[cfg(test)]
//extern crate mock_it;
//
//use bot::commands::dataobjects::TrackingState;
//use bot::commands::services::TrackingStateService;
//
//use self::mock_it::Mock;
//
//use std::io::Error;
//
//#[derive(Clone)]
//pub struct TrackingStateServiceMock<'a>{
//    state: Mock<(&'a str),Result<TrackingState,Error>>,
//}
//
//impl <'a>TrackingStateServiceMock<'a>{
//
//    pub fn new()  -> TrackingStateServiceMock<'a>{
//        TrackingStateServiceMock{state: Mock::new(Ok(TrackingState::new_empty()))}
//    }
//}
//
////impl<'a>  TrackingStateService for TrackingStateServiceMock<'a>{
////
////    fn state(&self, tracking_code: &str) -> Result<TrackingState,Error>{
////
////    }
////
////}

use bot::commands::services::TrackingStateService;
use bot::commands::dataobjects::TrackingState;

use std::io::{Error,ErrorKind};

pub struct TrackingStateServiceMock;

/// Mock of the TrackingStateService
impl TrackingStateServiceMock{
    pub fn new() -> TrackingStateServiceMock{
        TrackingStateServiceMock
    }
}

impl TrackingStateService for TrackingStateServiceMock{

    /// returns only states with the code 0000 and 1111, otherwise
    /// it returns Option::None, except at 9999, it returns an io Error
    fn state(&self, tracking_code: &str) -> Result<Option<TrackingState>,Error>{
        let delivery_adr =  String::from("deliv str. 21");
        let delivered_to= String::from("Professor Professorson");
        let state = String::from("on the way");
        match tracking_code {
            "0000"=> Ok(Some(TrackingState::new(0000,delivery_adr,delivered_to,state))),
            "1111" => Ok(Some(TrackingState::new(1111,delivery_adr,delivered_to,state))),
            "9999"=> Err(Error::new(ErrorKind::InvalidData, "invalid code")),
            _ =>Ok(None),
        }
    }

}