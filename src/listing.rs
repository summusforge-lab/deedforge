#[derive(Debug, PartialEq)]
pub enum ListingType {
    Sale,
    Rent,
}

#[derive(Debug, PartialEq)]
pub enum ListingStatus {
    Active,
    Closed,
}

pub struct Listing {
    pub id: i32,
    pub property_id: i32,
    pub listing_type: ListingType,
    pub price: u64,
    pub realtor: String,
    pub status: ListingStatus,
}
