use crate::listing::{Listing, ListingStatus, ListingType};
use std::fmt;

pub struct PropertyItem {
    pub id: i32,
    pub name: String,
    pub owner: String,
    pub doc_hash: String,
}
#[derive(Debug, PartialEq)]
pub enum RegistryError {
    PropertyAlreadyExists,
    PropertyNotFound,
    UnauthorizedTransfer,
    PropertyInvalid,
    PropertyAlreadyActive,
    ListingNotFound,
    ListingInvalid,
}
pub struct TransferRecord {
    pub property_id: i32,
    pub from: String,
    pub to: String,
}
#[derive(Default)]
pub struct Registry {
    properties: Vec<PropertyItem>,
    transfer_history: Vec<TransferRecord>,
    listings: Vec<Listing>,
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistryError::PropertyAlreadyExists => {
                write!(f, "property already exists")
            }
            RegistryError::PropertyNotFound => {
                write!(f, "property not found in the registry")
            }
            RegistryError::UnauthorizedTransfer => {
                write!(f, "unauthorized transfer")
            }
            RegistryError::PropertyInvalid => {
                write!(f, "invalid property")
            }
            RegistryError::ListingNotFound => {
                write!(f, "listing not found")
            }
            RegistryError::PropertyAlreadyActive => {
                write!(f, "property is active already")
            }
            RegistryError::ListingInvalid => {
                write!(f, "Listing is invalid")
            }
        }
    }
}

impl Registry {
    //SECTION: Initialization
    pub fn new() -> Self {
        Self::default()
    }
    //SECTION: work with propery
    //MARK: Validate data
    fn is_valid_property(&self, item: &PropertyItem) -> bool {
        !item.name.is_empty() && !item.owner.is_empty() && !item.doc_hash.is_empty()
    }
    //MARK: Add new property
    pub fn add_property(&mut self, item: PropertyItem) -> Result<(), RegistryError> {
        if self.properties.iter().any(|prop| prop.id == item.id) {
            return Err(RegistryError::PropertyAlreadyExists);
        }
        if !self.is_valid_property(&item) {
            return Err(RegistryError::PropertyInvalid);
        }
        self.properties.push(item);
        Ok(())
    }
    pub fn count(&self) -> usize {
        self.properties.len()
    }
    pub fn list(&self) -> &[PropertyItem] {
        &self.properties
    }
    //MARK: Transfer property to another owner
    pub fn transfer(
        &mut self,
        property_id: i32,
        sender: String,
        new_owner: String,
    ) -> Result<(), RegistryError> {
        let property = self
            .properties
            .iter_mut()
            .find(|prop| prop.id == property_id);

        match property {
            Some(property) => {
                if property.owner == sender {
                    //NOTE: change owner
                    property.owner = new_owner.clone();
                    //NOTE: add to history
                    self.transfer_history.push(TransferRecord {
                        property_id,
                        from: sender,
                        to: new_owner,
                    });
                    Ok(())
                } else {
                    Err(RegistryError::UnauthorizedTransfer)
                }
            }
            None => Err(RegistryError::PropertyNotFound),
        }
    }
    //MARK: Get property by ID
    pub fn get_property(&self, property_id: i32) -> Option<&PropertyItem> {
        self.properties.iter().find(|prop| prop.id == property_id)
    }
    //MARK: Transfers' history
    pub fn history(&self, property_id: i32) -> Vec<&TransferRecord> {
        self.transfer_history
            .iter()
            .filter(|record| record.property_id == property_id)
            .collect()
    }
    //SECTION: work with listings
    //MARK: get listings
    pub fn listings(&self) -> &[Listing] {
        &self.listings
    }
    //MARK: get listing by id
    pub fn get_listing(&self, id: i32) -> Option<&Listing> {
        self.listings.iter().find(|list| list.id == id)
    }
    //MARK: Validate listing
    fn listing_valid(&self, listing_item: &Listing) -> bool {
        listing_item.price > 0 && !listing_item.realtor.is_empty()
    }
    //MARK: Add listings
    pub fn add_listing(
        &mut self,
        property_id: i32,
        listing_type: ListingType,
        price: u64,
        realtor: String,
    ) -> Result<(), RegistryError> {
        if !self.properties.iter().any(|prop| prop.id == property_id) {
            return Err(RegistryError::PropertyNotFound);
        }

        let listing = Listing {
            id: self.listings().len() as i32,
            property_id,
            listing_type,
            price,
            realtor,
            status: ListingStatus::Active,
        };
        if self.listings().iter().any(|item| {
            item.property_id == listing.property_id && item.status == ListingStatus::Active
        }) {
            return Err(RegistryError::PropertyAlreadyActive);
        }
        if self.listing_valid(&listing) {
            self.listings.push(listing);
            Ok(())
        } else {
            Err(RegistryError::ListingInvalid)
        }
    }

    //MARK: Close listing
    pub fn close_listing(&mut self, listing_id: i32) -> Result<(), RegistryError> {
        let list = self
            .listings
            .iter_mut()
            .find(|listing| listing.id == listing_id);
        match list {
            Some(list) => {
                list.status = ListingStatus::Closed;
                Ok(())
            }
            None => Err(RegistryError::ListingNotFound),
        }
    }
}

//SECTION: Tests
#[cfg(test)]
#[path = "registry_tests.rs"]
mod registry_tests;
