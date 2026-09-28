# fory-practice

Hands-on exercises for Apache Fory (Rust, fory 1.7.5). Toolchain: Ubuntu `rustc-1.91` + `cargo-1.91`.

| Binary | Topic |
|---|---|
| `fory-practice` (src/main.rs) | `ForyStruct`, register, nested types, collections, round-trip |
| `ex1_enum_refs` | `ForyEnum`, shared `Rc` fields |
| `ex3_compatible` | `compatible(true/false)`, schema evolution |
| `ex4_bench` | Fory vs serde_json vs bincode (size and speed) |
| `ex5_shared` | Shared-reference graph: Fory vs bincode vs JSON |

Run one: `cargo-1.91 run --release --bin ex5_shared`
(with rustup, plain `cargo run --release --bin ex5_shared`)

Notes: the derive is `ForyStruct`; every type needs `register::<T>(id)`;
builder is `Fory::builder().compatible(..).track_ref(..).build()`.
