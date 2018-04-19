
use bot::commands::dataobjects::TrackingState;

use std::io::{Error,ErrorKind};
use std::error::Error as ErrorTrait;

use bot::configuration::config::DatabaseConfig;

use mysql::{Pool,Row};
use mysql;

/// Service for requesting the Delivery Status of a delivery
pub trait TrackingStateService{

    /// finds the tracking information of the delivery with the given code
    fn state(&self, tracking_code: &str) -> Result<Option<TrackingState>,Error>;

}

pub struct TrackingStateServiceImpl{
    pool: Pool,
}

impl TrackingStateServiceImpl{

    pub fn new(config: &DatabaseConfig) -> TrackingStateServiceImpl {
        let login = format!("mysql://{}:{}@{}:{}",
                            config.user(),config.password().unwrap(),config.host(),config.port());
        let pool = Pool::new(login).unwrap();
        TrackingStateServiceImpl{pool}
    }

    fn map_to_io_err(e: mysql::Error) -> Error{
        println!("Error in TrackingStateServiceImpl: {:#?}", &e );
        let descr = e.description();
        Error::new(ErrorKind::NotConnected, descr)
    }

}

impl TrackingStateService for TrackingStateServiceImpl{

    fn state(&self, tracking_code: &str) -> Result<Option<TrackingState>,Error>{

        let params = params!{
                "access_key" => tracking_code,
            };

        let mut selected_states: Vec<TrackingState> =
            self.pool.prep_exec("SELECT access_key, delivery_address, delivered_to,order_status from shop.order WHERE access_key = (:access_key)", params)
                .map(|result| {
                    result.map(|x| x.unwrap()).map(|row| {
                        let (access_key, del_adr,del_to,state) = mysql::from_row(row);
                        TrackingState::new(access_key,del_adr,del_to,state)
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<TrackingState>`
                }).unwrap(); // Unwrap `Vec<TrackingState>`

        if selected_states.is_empty(){
            return Ok(None);
        }

        let state = selected_states.pop().unwrap();
        Ok(Some(state))
    }
}