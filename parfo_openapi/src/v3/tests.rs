use std::error::Error;
use std::fs::File;

use serde_json;

use super::*;

#[test]
fn json_deserialize() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/fixtures/v3_0_petstore.json")?;

    let spec: Spec = serde_json::from_reader(file)?;

    print!("{:?}", spec);

    Ok(())
}

#[test]
fn yaml_deserialize() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/fixtures/v3_0_s3.yaml")?;

    let spec: Spec = serde_yaml::from_reader(file)?;

    print!("{:?}", spec);

    Ok(())
}
