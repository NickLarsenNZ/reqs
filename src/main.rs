use std::collections::HashMap;

use crate::config::{Action, Config, Req};

mod config;

fn main() {
    let config = Config {
        reqs: vec![Req {
            // id: todo!(),
            // method: todo!(),
            body: Some(config::Body::BodyJson(HashMap::from([
                ("client_id".to_owned(), "abc".to_owned()),
                ("client_secret".to_owned(), "xyz".to_owned()),
            ]))),
            action: Some(Action::Print("".to_owned())),
            ..Default::default()
        }],
        ..Default::default()
    };

    let yaml = serde_yaml::to_string(&config).expect("to be able to serialize");

    println!("{yaml}");
}
