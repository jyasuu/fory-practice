use fory::{Fory, ForyStruct};
use serde::{Deserialize, Serialize};
use std::hint::black_box;
use std::rc::Rc;
use std::time::Instant;

#[derive(ForyStruct, Serialize, Deserialize, Debug, Clone)]
struct Customer {
    id: i64,
    name: String,
    address: String,
    notes: Vec<String>,
}

#[derive(ForyStruct, Serialize, Deserialize, Debug, Clone)]
struct Order {
    id: i64,
    amount: f64,
    customer: Rc<Customer>,
}

fn build(orders: usize, customers: usize) -> Vec<Order> {
    let cs: Vec<Rc<Customer>> = (0..customers)
        .map(|i| Rc::new(Customer {
            id: i as i64,
            name: format!("customer-{i}"),
            address: "No. 1, Zhongshan Rd., Changhua City, Taiwan 500".into(),
            notes: (0..5).map(|n| format!("note-{n}-lorem-ipsum-dolor-sit-amet")).collect(),
        }))
        .collect();
    (0..orders)
        .map(|i| Order { id: i as i64, amount: i as f64 * 1.5, customer: cs[i % customers].clone() })
        .collect()
}

fn distinct(v: &[Order]) -> usize {
    let mut p: Vec<*const Customer> = v.iter().map(|o| Rc::as_ptr(&o.customer)).collect();
    p.sort();
    p.dedup();
    p.len()
}

fn time<T>(iters: usize, f: impl Fn() -> T) -> f64 {
    let t = Instant::now();
    for _ in 0..iters { black_box(f()); }
    t.elapsed().as_micros() as f64 / iters as f64
}

fn main() {
    let (orders, customers, iters) = (1000, 10, 200);
    let data = build(orders, customers);
    println!("{orders} orders sharing {customers} customers (distinct in memory: {})\n", distinct(&data));

    let mut fory = Fory::default();
    fory.register::<Customer>(1).unwrap();
    fory.register::<Order>(2).unwrap();

    let f_bytes = fory.serialize(&data).unwrap();
    let b_bytes = bincode::serialize(&data).unwrap();
    let j_bytes = serde_json::to_vec(&data).unwrap();

    let f_back: Vec<Order> = fory.deserialize(&f_bytes).unwrap();
    let b_back: Vec<Order> = bincode::deserialize(&b_bytes).unwrap();
    println!("{:<12} {:>9} {:>12} {:>12} {:>16}", "codec", "bytes", "ser (us)", "de (us)", "distinct after decode");
    println!("{:<12} {:>9} {:>12.0} {:>12.0} {:>16}", "fory", f_bytes.len(),
        time(iters, || fory.serialize(&data).unwrap()),
        time(iters, || fory.deserialize::<Vec<Order>>(&f_bytes).unwrap()),
        distinct(&f_back));
    println!("{:<12} {:>9} {:>12.0} {:>12.0} {:>16}", "bincode", b_bytes.len(),
        time(iters, || bincode::serialize(&data).unwrap()),
        time(iters, || bincode::deserialize::<Vec<Order>>(&b_bytes).unwrap()),
        distinct(&b_back));
    println!("{:<12} {:>9} {:>12.0} {:>12.0} {:>16}", "serde_json", j_bytes.len(),
        time(iters, || serde_json::to_vec(&data).unwrap()),
        time(iters, || serde_json::from_slice::<Vec<Order>>(&j_bytes).unwrap()),
        distinct(&serde_json::from_slice::<Vec<Order>>(&j_bytes).unwrap()));
}
