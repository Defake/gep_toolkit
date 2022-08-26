use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use serde::{Serialize};
use serde::de::DeserializeOwned;

pub fn serialize<T: Serialize>(obj: &T) -> std::io::Result<Vec<u8>> {
    let encoded: Vec<u8> = bincode::serialize(&obj).unwrap();
    Ok(encoded)
}

pub fn serialize_to_file<T: Serialize>(filename: &str, obj: &T) -> std::io::Result<()> {
    let encoded = serialize(obj)?;
    File::create(filename)?
        .write_all(&encoded)?;
    Ok(())
}

pub fn deserialize_from_file<T: DeserializeOwned>(filename: &str) -> std::io::Result<T> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(bincode::deserialize_from(reader).unwrap())
}
