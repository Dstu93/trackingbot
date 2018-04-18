
/// Data Struct to represent an State of the package delivery
#[derive(Debug,Hash,Clone,Ord, PartialOrd, PartialEq,Eq)]
pub struct TrackingState{   //TODO Liefer und Bestellzeitpunkt hinzufügen... benötigt seperate crate...
    access_key: u64,
    delivery_address: String,
    delivered_to: String,
    order_state: String,
}

impl TrackingState{

    /// unique access key of this delivery
    pub fn access_key(&self) -> u64{
        self.access_key
    }

    /// address for delivery
    pub fn delivery_address(&self) -> &String{
        &self.delivery_address
    }

    pub fn delivered_to(&self) -> &String{
        &self.delivered_to
    }

    /// State of the order
    pub fn state(&self) -> &String{
        &self.order_state
    }

}