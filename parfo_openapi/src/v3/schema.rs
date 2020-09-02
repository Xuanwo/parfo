use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_value::Value;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Spec {
    pub openapi: String,
    pub info: Option<Info>,

    pub paths: BTreeMap<String, PathItem>,
    pub components: Option<Components>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Info {
    pub version: String,

    #[serde(flatten)]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Object<T> {
    Origin(T),
    Reference(Reference),
    Empty {},
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub ref_: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct PathItem {
    pub summary: Option<String>,
    pub description: Option<String>,

    pub parameters: Option<Vec<Object<Parameter>>>,

    #[serde(flatten)]
    pub operations: BTreeMap<String, Operation>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Components {
    pub schemas: Option<BTreeMap<String, Object<Schema>>>,
    pub responses: Option<BTreeMap<String, Object<Response>>>,
    pub parameters: Option<BTreeMap<String, Object<Parameter>>>,
    pub request_bodies: Option<BTreeMap<String, RequestBody>>,
    pub headers: Option<BTreeMap<String, Header>>,
}

// ref: https://swagger.io/specification/
//   - integer as a type is also supported and is defined as a JSON number without a fraction or exponent part.
//   - null is not supported as a type (see nullable for an alternative solution).
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SchemaType {
    Boolean,
    Object,
    Array,
    Number,
    String,
    Integer,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Schema {
    #[serde(rename = "type")]
    pub type_: SchemaType,
    pub items: Option<Box<Object<Schema>>>,
    pub format: Option<String>,
    pub properties: Option<BTreeMap<String, Object<Schema>>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Response {
    pub description: String,
    pub headers: Option<BTreeMap<String, Object<Header>>>,
    pub content: Option<BTreeMap<String, MediaType>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub in_: String,
    pub required: Option<bool>,
    // TODO: support content here.
    pub schema: Schema,
    pub style: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct RequestBody {
    pub content: BTreeMap<String, MediaType>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Header {
    pub description: Option<String>,
    pub schema: Schema,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Operation {
    pub summary: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "operationId")]
    pub operation_id: String,
    pub parameters: Option<Vec<Object<Parameter>>>,
    #[serde(rename = "requestBody")]
    pub request_body: Option<RequestBody>,
    pub responses: Option<BTreeMap<String, Response>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct MediaType {
    pub schema: Object<Schema>,
}
