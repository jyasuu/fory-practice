use fory::{Error, Fory, ForyEnum, ForyStruct};
use std::rc::Rc;

// Exercise 1: enum
#[derive(ForyEnum, Debug, PartialEq, Clone, Default)]
enum Role {
    #[default]
    Member,
    Admin,
}

#[derive(ForyStruct, Debug, Clone)]
struct Address {
    city: String,
}

// Exercise 2: shared reference
#[derive(ForyStruct, Debug)]
struct Person {
    name: String,
    role: Role,
    home: Rc<Address>,
    office: Rc<Address>,
}

fn main() -> Result<(), Error> {
    let mut fory = Fory::builder().track_ref(true).build();
    fory.register::<Role>(1)?;
    fory.register::<Address>(2)?;
    fory.register::<Person>(3)?;

    let shared = Rc::new(Address { city: "Changhua".into() });
    let p = Person {
        name: "Jyasu".into(),
        role: Role::Admin,
        home: shared.clone(),
        office: shared.clone(),
    };

    let bytes = fory.serialize(&p)?;
    let back: Person = fory.deserialize(&bytes)?;
    println!("role = {:?}, bytes = {}", back.role, bytes.len());
    println!("same Rc after decode? {}", Rc::ptr_eq(&back.home, &back.office));

    // Compare: without ref tracking, the object is written twice
    let mut plain = Fory::default();
    plain.register::<Role>(1)?;
    plain.register::<Address>(2)?;
    plain.register::<Person>(3)?;
    let b2 = plain.serialize(&p)?;
    let back2: Person = plain.deserialize(&b2)?;
    println!("no track_ref: bytes = {}, same Rc? {}", b2.len(), Rc::ptr_eq(&back2.home, &back2.office));
    Ok(())
}
