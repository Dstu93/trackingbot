
use bot::commands::dataobjects::TrackingState;

use std::io::Error;

/// Service for requesting the Delivery Status of a delivery
pub trait TrackingStateService{

    /// finds the tracking information of the delivery with the given code
    fn state(&self, tracking_code: &str) -> Result<TrackingState,Error>;

}