use std::{collections::BTreeMap, env, fs, sync::Arc};

use adapter::Adapter;

use trustfall::{execute_query, Schema, TransparentValue};

mod adapter;

fn main() {
    let schema = Schema::parse(
        fs::read_to_string(r#"/workspace/github_adapter/src/schema.graphql"#).unwrap(),
    )
    .unwrap();

    let query = r#"
    query {
        Repository(owner: "ziglang") {
            owner {
                name @tag(name: "repo_owner")
            }

            issue {
                opened_by {
                    issue_opener_name: name @filter(op: "!=", value: ["%repo_owner"]) @tag
                }

                name @output

                state @filter(op: "=", value: ["$open"])

                reactions {
                    total @filter(op: ">=", value: ["$ten"])
                }

                comment {
                    by {
                        name @filter(op: "=", value: ["%issue_opener_name"])
                    }

                    message @output

                    reactions {
                        plus_one @filter(op: ">=", value: ["$ten"])
                    }   
                }
            }
        }
    }
    "#;

    let mut args: BTreeMap<Arc<str>, TransparentValue> = BTreeMap::new();

    args.insert("ten".into(), TransparentValue::Int64(10));
    args.insert("open".into(), TransparentValue::String("open".to_owned()));

    let adapter = Arc::new(Adapter {});

    for data_item in execute_query(&schema, adapter, query, args)
        .expect("not a legal query")
        .take(
            env::var("TAKE_COUNT")
                .unwrap_or("1".to_owned())
                .parse()
                .expect("to be able to parse env['TAKE_COUNT']"),
        )
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
