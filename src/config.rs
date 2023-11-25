use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use url::Url;

/// The config for the invocation of `reqs`.
/// You might have many of these defined to run various http workflows, such as:
/// - rest api testing
/// - oauth token retrieval
#[derive(Debug, SmartDefault, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    /// The base URL to use for requests to make life easier.
    /// Requests can override this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<Url>,

    /// Number of concurrent requests. Default = 2
    #[default(Some(2))]
    pub max_parallel: Option<u8>,

    /// A list of requests to run.
    /// Requests with no dependencies will run
    pub reqs: Vec<Req>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Req {
    /// An optional id for referencing in other tasks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// HTTP method and url to use
    #[serde(flatten)]
    pub method: Method,

    /// HTTP body data to send
    #[serde(flatten)]
    pub body: Option<Body>,

    /// Action to do with the result, for example:
    /// - No action (default)
    /// - Print to STDOUT
    /// - Write to file
    #[serde(flatten)]
    pub action: Option<Action>,
}

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
// later to support upper case methods GET
// #[serde(deserialize_with = "case_insensitive")]
pub enum Method {
    #[default]
    Get(#[default = "/"] String),
    Post(String),
}

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Body {
    #[default]
    Body(String), // todo: make this a templatable expression
    BodyJson(HashMap<String, String>), // todo: make this String: templatable expressions
}

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    #[default]
    NoOp(String),

    /// If no value is provided, the whole body is printed
    /// _Or maybe this should be headers and body?_
    // todo: make this a templatable expression
    Print(String),

    /// Path to append output to. If no file exists, one will be created.
    /// You can use make use of interpolation such as `${{ run_id }}` and `${{ job_id }}`
    WriteFile(PathBuf),
}
