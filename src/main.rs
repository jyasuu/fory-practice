use fory::{Error, Fory, ForyStruct};
use std::collections::HashMap;

// Lesson 1: a basic struct
#[derive(ForyStruct, Debug, PartialEq, Clone)]
struct Address {
    city: String,
    zip: i32,
}

// Lesson 2: nesting, collections, Option
#[derive(ForyStruct, Debug, PartialEq, Clone)]
struct User {
    id: i64,
    name: String,
    active: bool,
    tags: Vec<String>,
    scores: HashMap<String, f64>,
    address: Address,
    nickname: Option<String>,
}

fn main() -> Result<(), Error> {
    let mut fory = Fory::default();
    // Types must be registered with a numeric id (or a name) before use.
    fory.register::<Address>(100)?;
    fory.register::<User>(101)?;

    let user = User {
        id: 42,
        name: "Jyasu".into(),
        active: true,
        tags: vec!["rust".into(), "fory".into()],
        scores: HashMap::from([("math".into(), 98.5), ("cs".into(), 100.0)]),
        address: Address { city: "Changhua".into(), zip: 500 },
        nickname: None,
    };

    let bytes = fory.serialize(&user)?;
    println!("serialized {} bytes", bytes.len());

    let back: User = fory.deserialize(&bytes)?;
    assert_eq!(user, back);
    println!("round-trip OK: {:?}", back);

    // Lesson 3: top-level collections work too
    let list = vec![user.address.clone(), Address { city: "Taipei".into(), zip: 100 }];
    let b = fory.serialize(&list)?;
    let l2: Vec<Address> = fory.deserialize(&b)?;
    assert_eq!(list, l2);
    println!("vec round-trip OK ({} bytes)", b.len());

    Ok(())
}
