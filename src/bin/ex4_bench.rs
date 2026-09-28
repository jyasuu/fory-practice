use fory::{Fory, ForyStruct};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hint::black_box;
use std::time::Instant;

#[derive(ForyStruct, Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Address {
    city: String,
    zip: i32,
}

#[derive(ForyStruct, Serialize, Deserialize, Debug, PartialEq, Clone)]
struct User {
    id: i64,
    name: String,
    active: bool,
    tags: Vec<String>,
    scores: HashMap<String, f64>,
    address: Address,
    nickname: Option<String>,
}

fn sample(i: i64) -> User {
    User {
        id: i,
        name: format!("user-{i}"),
        active: i % 2 == 0,
        tags: vec!["rust".into(), "fory".into(), "bench".into()],
        scores: HashMap::from([("math".into(), 98.5), ("cs".into(), 100.0)]),
        address: Address { city: "Changhua".into(), zip: 500 },
        nickname: if i % 3 == 0 { Some("nick".into()) } else { None },
    }
}

fn bench<T>(name: &str, iters: usize, ser: impl Fn(&User) -> Vec<u8>, de: impl Fn(&[u8]) -> T) {
    let data: Vec<User> = (0..1000).map(sample).collect();
    let size: usize = data.iter().map(|u| ser(u).len()).sum::<usize>() / data.len();

    let t = Instant::now();
    for _ in 0..iters {
        for u in &data {
            black_box(ser(black_box(u)));
        }
    }
    let ser_t = t.elapsed();

    let encoded: Vec<Vec<u8>> = data.iter().map(|u| ser(u)).collect();
    let t = Instant::now();
    for _ in 0..iters {
        for b in &encoded {
            black_box(de(black_box(b)));
        }
    }
    let de_t = t.elapsed();

    let n = (iters * data.len()) as f64;
    println!(
        "{name:<18} {size:>5} B   ser {:>7.0} ns/op   de {:>7.0} ns/op",
        ser_t.as_nanos() as f64 / n,
        de_t.as_nanos() as f64 / n
    );
}

fn main() {
    let iters = 200;

    let mut fory = Fory::default();
    fory.register::<Address>(100).unwrap();
    fory.register::<User>(101).unwrap();

    let mut fory_c = Fory::builder().compatible(true).build();
    fory_c.register::<Address>(100).unwrap();
    fory_c.register::<User>(101).unwrap();

    // sanity: every codec round-trips
    let u = sample(3);
    assert_eq!(u, fory.deserialize::<User>(&fory.serialize(&u).unwrap()).unwrap());
    assert_eq!(u, fory_c.deserialize::<User>(&fory_c.serialize(&u).unwrap()).unwrap());
    assert_eq!(u, serde_json::from_slice::<User>(&serde_json::to_vec(&u).unwrap()).unwrap());
    assert_eq!(u, bincode::deserialize::<User>(&bincode::serialize(&u).unwrap()).unwrap());

    println!("{:<18} {:>7}   {:>18}   {:>17}", "codec", "avg size", "serialize", "deserialize");
    bench("fory", iters,
        |u| fory.serialize(u).unwrap(),
        |b| fory.deserialize::<User>(b).unwrap());
    bench("fory (compatible)", iters,
        |u| fory_c.serialize(u).unwrap(),
        |b| fory_c.deserialize::<User>(b).unwrap());
    bench("serde_json", iters,
        |u| serde_json::to_vec(u).unwrap(),
        |b| serde_json::from_slice::<User>(b).unwrap());
    bench("bincode", iters,
        |u| bincode::serialize(u).unwrap(),
        |b| bincode::deserialize::<User>(b).unwrap());
}
