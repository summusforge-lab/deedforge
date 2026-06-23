use deedforge::registry::Registry;

fn main() {
    println!("Hello, world!\nLet's start");
    let mut registry = Registry::new();

    let item = deedforge::registry::PropertyItem {
        id: 0,
        name: String::from("First property"),
        owner: String::from("0x0"),
        doc_hash: String::from("0x0"),
    };

    let add_result = registry.add_property(item);
    match add_result {
        Ok(()) => {
            println!("property added");
        }
        Err(error) => println!("Error: {:?}", error),
    }

    println!("Complete...");
}
