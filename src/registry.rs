use crate::listing::{Listing, ListingStatus};

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
    listing: Vec<Listing>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
            transfer_history: Vec::new(),
            listing: Vec::new(),
        }
    }
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
    pub fn get(&self, property_id: i32) -> Option<&PropertyItem> {
        self.properties.iter().find(|prop| prop.id == property_id)
    }
    //MARK: Transfers' history
    pub fn history(&self, property_id: i32) -> Vec<&TransferRecord> {
        self.transfer_history
            .iter()
            .filter(|record| record.property_id == property_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;

    fn prepare_data(item_number: usize) -> Registry {
        let mut registry = Registry::new();

        for i in 0..item_number {
            let name = format!("Name {}", i);
            let item = PropertyItem {
                id: i as i32,
                name,
                owner: String::from("0x0"),
                doc_hash: String::from("0x0"),
            };
            let _ = registry.add_property(item);
        }

        registry
    }
    #[test]
    fn adds_property_to_registry() {
        let mut registry = prepare_data(5);
        let initial_count = registry.count();
        let new_item = PropertyItem {
            id: registry.list().len() as i32,
            name: String::from("New item"),
            owner: String::from("0x0"),
            doc_hash: String::from("0x0"),
        };
        let result = registry.add_property(new_item);
        assert!(result.is_ok());
        assert_eq!(registry.count(), initial_count + 1);
        assert_eq!(
            registry
                .get(initial_count as i32)
                .expect("new property should exist")
                .name,
            "New item"
        );
    }
    #[test]
    fn rejects_duplicate_property_id() {
        let mut registry = prepare_data(5);
        let initial_count = registry.count();
        let new_item = PropertyItem {
            id: 0_i32,
            name: String::from("New item"),
            owner: String::from("0x0"),
            doc_hash: String::from("0x0"),
        };
        let result = registry.add_property(new_item);
        assert_matches!(result, Err(RegistryError::PropertyAlreadyExists));
        assert_eq!(registry.count(), initial_count);
    }
    #[test]
    fn owner_can_transfer_property() {
        let mut registry = prepare_data(5);
        let result = registry.transfer(0, String::from("0x0"), String::from("0x1"));
        assert!(result.is_ok());
        assert_eq!(
            registry
                .get(0)
                .expect("transferred property should exist")
                .owner,
            "0x1"
        );
    }
    #[test]
    fn non_owner_cannot_transfer_property() {
        let mut registry = prepare_data(5);
        let result = registry.transfer(0, String::from("0x2"), String::from("0x1"));
        assert_matches!(result, Err(RegistryError::UnauthorizedTransfer));
        assert_eq!(
            registry.get(0).expect("property should still exist").owner,
            "0x0"
        );
    }
    #[test]
    fn cannot_transfer_missing_property() {
        let mut registry = prepare_data(5);
        let result = registry.transfer(6, String::from("0x2"), String::from("0x1"));
        assert_matches!(result, Err(RegistryError::PropertyNotFound));
    }
    #[test]
    fn get_existing_property() {
        let registry = prepare_data(5);
        let property = registry.get(0);
        let property = property.expect("property should exist");
        assert_eq!(property.id, 0);
        assert_eq!(property.name, "Name 0");
        assert_eq!(property.owner, "0x0");
    }
    #[test]
    fn get_unknown_property() {
        let registry = prepare_data(5);
        let property = registry.get(9);
        assert!(property.is_none());
    }
    #[test]
    fn empty_property_field() {
        let mut registry = prepare_data(5);
        //let initial_count = registry.count();
        let new_item = PropertyItem {
            id: registry.list().len() as i32,
            name: String::from(""),
            owner: String::from("0xf"),
            doc_hash: String::from("0x0"),
        };
        let result = registry.add_property(new_item);
        assert_matches!(result, Err(RegistryError::PropertyInvalid));
        let new_item2 = PropertyItem {
            id: registry.list().len() as i32,
            name: String::from("0xf"),
            owner: String::new(),
            doc_hash: String::from("0x0"),
        };
        let result = registry.add_property(new_item2);
        assert_matches!(result, Err(RegistryError::PropertyInvalid));

        let new_item3 = PropertyItem {
            id: registry.list().len() as i32,
            name: String::from("0xf"),
            owner: String::from("0xf"),
            doc_hash: String::new(),
        };
        let result = registry.add_property(new_item3);
        assert_matches!(result, Err(RegistryError::PropertyInvalid));
    }
    //NOTE: transfer history tests
    #[test]
    fn transfer_added_to_history() {
        let mut registry = prepare_data(5);
        let _ = registry.transfer(0, String::from("0x0"), String::from("0x1"));
        let history = registry.history(0);
        assert_ne!(history.len(), 0);
        assert_eq!(history.len(), 1);
        let record = history[0];
        assert_eq!(record.from, String::from("0x0"));
        assert_eq!(record.to, String::from("0x1"));
    }
    #[test]
    fn transfer_missing_property() {
        let mut registry = prepare_data(5);
        let _ = registry.transfer(6, String::from("0x0"), String::from("0x1"));
        let history = registry.history(6);
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn unauthorized_transfer() {
        let mut registry = prepare_data(5);
        let _ = registry.transfer(0, String::from("0x2"), String::from("0x1"));
        let history = registry.history(0);
        assert_eq!(history.len(), 0);
    }
}
