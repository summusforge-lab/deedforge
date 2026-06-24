use super::*;
use crate::listing::{Listing, ListingStatus, ListingType};
use std::assert_matches;

//SECTION: Supported functions
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
fn create_listing() -> (Listing, Listing) {
    let list_1 = Listing {
        id: 0,
        property_id: 0,
        listing_type: ListingType::Sale,
        price: 1000,
        realtor: String::from("Realtor 1"),
        status: ListingStatus::Active,
    };
    let list_2 = Listing {
        id: 0,
        property_id: 1,
        listing_type: ListingType::Rent,
        price: 100,
        realtor: String::from("Realtor 1"),
        status: ListingStatus::Active,
    };
    (list_1, list_2)
}

//SECTION: Tests
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
            .get_property(initial_count as i32)
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
            .get_property(0)
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
        registry
            .get_property(0)
            .expect("property should still exist")
            .owner,
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
    let property = registry.get_property(0);
    let property = property.expect("property should exist");
    assert_eq!(property.id, 0);
    assert_eq!(property.name, "Name 0");
    assert_eq!(property.owner, "0x0");
}
#[test]
fn get_unknown_property() {
    let registry = prepare_data(5);
    let property = registry.get_property(9);
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
//NOTE: Listing tests
#[test]
fn creates_sale_listing_for_existing_property() {
    let mut registry = prepare_data(5);
    assert_eq!(registry.listings().len(), 0);
    let lists = create_listing();
    let list = lists.0;
    assert!(registry.get_property(list.property_id).is_some());
    let result = registry.add_listing(
        list.property_id,
        list.listing_type,
        list.price,
        list.realtor,
    );
    assert_matches!(result, Ok(()));
    assert_eq!(registry.listings().len(), 1);
    assert_eq!(registry.listings()[0].listing_type, ListingType::Sale);
}

#[test]
fn creates_rent_listing_for_existing_property() {
    let mut registry = prepare_data(5);
    assert_eq!(registry.listings().len(), 0);
    let lists = create_listing();
    let list = lists.1;
    assert!(registry.get_property(list.property_id).is_some());
    let result = registry.add_listing(
        list.property_id,
        list.listing_type,
        list.price,
        list.realtor,
    );
    assert_matches!(result, Ok(()));
    assert_eq!(registry.listings().len(), 1);
    assert_eq!(registry.listings()[0].listing_type, ListingType::Rent);
}

#[test]
fn rejects_listing_for_missing_property() {
    let mut registry = prepare_data(5);
    assert_eq!(registry.listings().len(), 0);
    let list = Listing {
        id: 0,
        property_id: 10,
        listing_type: ListingType::Sale,
        price: 100,
        realtor: String::from("Realtor 1"),
        status: ListingStatus::Active,
    };
    let result = registry.add_listing(
        list.property_id,
        list.listing_type,
        list.price,
        list.realtor,
    );
    assert_matches!(result, Err(RegistryError::PropertyNotFound));
}

#[test]
fn listing_starts_as_active() {
    let mut registry = prepare_data(5);
    assert_eq!(registry.listings().len(), 0);
    let lists = create_listing();
    let list_1 = lists.0;
    let list_2 = lists.1;

    let result = registry.add_listing(
        list_1.property_id,
        list_1.listing_type,
        list_1.price,
        list_1.realtor,
    );
    assert_matches!(result, Ok(()));

    let result_2 = registry.add_listing(
        list_2.property_id,
        list_2.listing_type,
        list_2.price,
        list_2.realtor,
    );
    assert_matches!(result_2, Ok(()));

    assert_eq!(registry.listings().len(), 2);
    assert_eq!(registry.listings()[0].status, ListingStatus::Active);
    assert_eq!(registry.listings()[1].status, ListingStatus::Active);
}

#[test]
fn close_listing_test() {
    let mut registry = prepare_data(5);
    assert_eq!(registry.listings().len(), 0);
    let lists = create_listing();
    let list = lists.0;
    assert!(registry.get_property(list.property_id).is_some());
    let result = registry.add_listing(
        list.property_id,
        list.listing_type,
        list.price,
        list.realtor,
    );
    assert_matches!(result, Ok(()));
    assert_eq!(registry.listings().len(), 1);
    let res = registry.close_listing(list.id);
    assert_matches!(res, Ok(()));
    assert_eq!(
        registry.get_listing(list.id).unwrap().status,
        ListingStatus::Closed
    );
}

#[test]
fn closes_listing_by_listing_id_not_property_id() {
    let mut registry = prepare_data(1);

    let first = registry.add_listing(0, ListingType::Sale, 1000, String::from("Realtor 1"));
    assert_matches!(first, Ok(()));

    let second = registry.add_listing(0, ListingType::Rent, 100, String::from("Realtor 2"));
    assert_matches!(second, Err(RegistryError::PropertyAlreadyActive));

    let result = registry.close_listing(0);
    assert_matches!(result, Ok(()));

    assert_eq!(
        registry.get_listing(0).unwrap().status,
        ListingStatus::Closed
    );

    let _ = registry.add_listing(0, ListingType::Rent, 100, String::from("Realtor 2"));
    let result = registry.close_listing(1);
    assert_matches!(result, Ok(()));
    assert_eq!(registry.listings()[1].status, ListingStatus::Closed);
}

#[test]
fn rejects_closing_missing_listing() {
    let mut registry = prepare_data(1);

    let result = registry.close_listing(0);

    assert_matches!(result, Err(RegistryError::ListingNotFound));
}
#[test]
fn reject_invalid_listing() {
    let mut registry = prepare_data(1);

    let result1 = registry.add_listing(0, ListingType::Sale, 0, String::from("Realtor 1"));
    assert_matches!(result1, Err(RegistryError::ListingInvalid));

    let result2 = registry.add_listing(0, ListingType::Sale, 1000, String::new());
    assert_matches!(result2, Err(RegistryError::ListingInvalid));
}
#[test]
fn reject_listing_active_property() {
    let mut registry = prepare_data(1);

    let _ = registry.add_listing(0, ListingType::Sale, 10000, String::from("Realtor 1"));
    let result2 = registry.add_listing(0, ListingType::Rent, 10, String::from("Realtor 2"));
    assert_matches!(result2, Err(RegistryError::PropertyAlreadyActive));
}
