#![feature(once_cell)]
use std::{collections::BTreeMap, fs, sync::Arc};

use adapter::Adapter;

use trustfall::{execute_query, Schema, TransparentValue};

mod adapter;

fn main() {
    let schema = Schema::parse(
        fs::read_to_string(
            r#"C:\Users\jason\Documents\git-proj\github_adapter\src\adapter\schema.graphql"#,
        )
        .unwrap(),
    )
    .unwrap();

    let query = r#"
    query {
        Repository(owner: "ziglang", name: "zig") {
            issue {
                name @output

                state @filter(op: "=", value: ["$open"])

                label {
                    name @filter(op: "=", value: ["$accepted"])
                }

                reactions {
                    total @filter(op: ">=", value: ["$fifty"])
                }
            }
        }
    }
    "#;

    let mut args: BTreeMap<Arc<str>, TransparentValue> = BTreeMap::new();

    args.insert(
        "accepted".into(),
        TransparentValue::String("accepted".to_owned()),
    );
    args.insert("fifty".into(), TransparentValue::Int64(50));
    args.insert("open".into(), TransparentValue::String("open".to_owned()));

    let adapter = Arc::new(Adapter {});

    for data_item in execute_query(&schema, adapter, query, args)
        .expect("not a legal query")
        .take(1)
    {
        // The default `FieldValue` JSON representation is explicit about its type, so we can get
        // reliable round-trip serialization of types tricky in JSON like integers and floats.
        //
        // The `TransparentValue` type is like `FieldValue` minus the explicit type representation,
        // so it's more like what we'd expect to normally find in JSON.
        let transparent: BTreeMap<_, TransparentValue> =
            data_item.into_iter().map(|(k, v)| (k, v.into())).collect();
        println!("\n{}", serde_json::to_string_pretty(&transparent).unwrap());
    }
}
