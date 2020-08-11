use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Spec {
    pub openapi: String,
    pub paths: BTreeMap<String, PathItem>,
    pub components: Option<Components>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Object<T> {
    Origin(T),
    Reference {
        #[serde(rename = "$ref")]
        ref_: String,
        description: Option<String>,
    },
    Empty {},
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    Get,
    Put,
    Post,
    Delete,
    Head,
    Patch,
    Options,
    Trace,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct PathItem {
    pub summary: Option<String>,
    pub description: Option<String>,

    pub parameters: Option<Vec<Object<Parameter>>>,

    #[serde(flatten)]
    pub operations: BTreeMap<Method, Operation>,
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
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SchemaType {
    Boolean,
    Object,
    Array,
    Number,
    String,
    Integer,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Schema {
    #[serde(rename = "type")]
    pub type_: SchemaType,
    pub items: Option<Box<Object<Schema>>>,
    pub format: Option<String>,
    pub properties: Option<BTreeMap<String, Object<Schema>>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Response {
    pub description: String,
    pub headers: Option<BTreeMap<String, Object<Header>>>,
    pub content: Option<BTreeMap<String, MediaType>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub in_: String,
    pub required: Option<bool>,
    // TODO: support content here.
    pub schema: Option<Schema>,
    pub style: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct RequestBody {
    pub content: BTreeMap<String, MediaType>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Header {
    pub description: Option<String>,
    pub schema: Schema,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Operation {
    pub summary: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "operationId")]
    pub operation_id: String,
    pub parameters: Option<Vec<Parameter>>,
    #[serde(rename = "requestBody")]
    pub request_body: Option<RequestBody>,
    pub responses: Option<BTreeMap<String, Response>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct MediaType {
    pub schema: Object<Schema>,
}
