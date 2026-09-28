use fory::{Fory, ForyStruct};

// Writer side: newer schema (v2) adds `email`, drops nothing
#[derive(ForyStruct, Debug, Default)]
struct UserV2 {
    id: i64,
    name: String,
    email: String,
}

// Reader side: older schema (v1) doesn't know `email`
#[derive(ForyStruct, Debug, Default)]
struct UserV1 {
    id: i64,
    name: String,
}

fn make(compatible: bool) -> (Fory, Fory) {
    let mut w = Fory::builder().compatible(compatible).build();
    let mut r = Fory::builder().compatible(compatible).build();
    // Same type id on both sides: that is what links the two schemas.
    w.register::<UserV2>(10).unwrap();
    r.register::<UserV1>(10).unwrap();
    (w, r)
}

fn main() {
    let v2 = UserV2 { id: 1, name: "Jyasu".into(), email: "j@example.com".into() };

    for compatible in [true, false] {
        let (writer, reader) = make(compatible);
        let bytes = writer.serialize(&v2).unwrap();
        println!("--- compatible = {compatible}: {} bytes", bytes.len());
        match reader.deserialize::<UserV1>(&bytes) {
            Ok(u) => println!("v2 -> v1 OK: {:?}", u),
            Err(e) => println!("v2 -> v1 FAILED: {e}"),
        }
    }

    // Reverse direction: old writer, new reader (missing field gets default)
    let (_, _) = make(true);
    let mut w1 = Fory::builder().compatible(true).build();
    let mut r2 = Fory::builder().compatible(true).build();
    w1.register::<UserV1>(10).unwrap();
    r2.register::<UserV2>(10).unwrap();
    let bytes = w1.serialize(&UserV1 { id: 2, name: "Old".into() }).unwrap();
    match r2.deserialize::<UserV2>(&bytes) {
        Ok(u) => println!("--- v1 -> v2 OK: {:?}", u),
        Err(e) => println!("--- v1 -> v2 FAILED: {e}"),
    }
}
