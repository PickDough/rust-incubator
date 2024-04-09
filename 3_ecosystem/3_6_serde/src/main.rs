use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("3_ecosystem/3_6_serde/request.json")?;
    let reader = BufReader::new(file);

    let req: Request = serde_json::from_reader(reader)?;

    println!("{}", req.debug.duration.to_string());

    let mut file = File::create("3_ecosystem/3_6_serde/request.toml")?;
    file.write_all(toml::to_string(&req)?.as_bytes())?;

    let mut file = File::create("3_ecosystem/3_6_serde/request.yaml")?;
    file.write_all(serde_yaml::to_string(&req)?.as_bytes())?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Request {
    #[serde(rename = "type")]
    pub type_enum: RequestType,
    pub stream: Stream,
    pub gifts: Vec<Gift>,
    pub debug: Debug,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RequestType {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Stream {
    pub user_id: String,
    pub is_private: bool,
    pub settings: u32,
    pub shard_url: Url,
    pub public_tariff: Tariff,
    pub private_tariff: Tariff,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Tariff {
    pub id: Option<u32>,
    pub price: Option<u32>,
    pub client_price: Option<u32>,
    #[serde(with = "durations_str_serde")]
    pub duration: Duration,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Gift {
    pub id: u32,
    pub price: u32,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Debug {
    #[serde(with = "durations_str_serde")]
    pub duration: Duration,
    pub at: DateTime<Utc>,
}

mod durations_str_serde {
    use chrono::Duration;
    use duration_str::deserialize_duration_chrono;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut d = duration.clone();
        let mut f = format!("{}h", duration.num_hours());

        d = d - Duration::try_hours(duration.num_hours()).unwrap();
        f.push_str(&format!(" {}m", d.num_minutes()));

        d = d - Duration::try_minutes(d.num_minutes()).unwrap();
        f.push_str(&format!(" {}s", d.num_seconds()));

        d = d - Duration::try_seconds(d.num_seconds()).unwrap();
        f.push_str(&format!(" {}ms", d.num_milliseconds()));

        serializer.serialize_str(&f)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_duration_chrono(deserializer)
    }
}

#[cfg(test)]
mod tests {
    use crate::Request;
    use std::fs::File;
    use std::io::{BufReader, Read, Write};

    #[test]
    fn test_json_to_toml() {
        let file = File::open("../../3_ecosystem/3_6_serde/request.json").unwrap();
        let reader = BufReader::new(file);
        let original_req: Request = serde_json::from_reader(reader).unwrap();

        let toml_string = toml::to_string(&original_req).unwrap();
        let mut file = File::create("../../3_ecosystem/3_6_serde/request.toml").unwrap();
        file.write_all(toml_string.as_bytes()).unwrap();

        let mut file = File::open("../../3_ecosystem/3_6_serde/request.toml").unwrap();
        let mut toml_string = String::new();
        file.read_to_string(&mut toml_string).unwrap();
        let deserialized_req: Request = toml::from_str(&toml_string).unwrap();

        assert_eq!(original_req, deserialized_req);
    }

    #[test]
    fn test_json_to_yaml() {
        let file = File::open("../../3_ecosystem/3_6_serde/request.json").unwrap();
        let reader = BufReader::new(file);
        let original_req: Request = serde_json::from_reader(reader).unwrap();

        let yaml_string = serde_yaml::to_string(&original_req).unwrap();
        let mut file = File::create("../../3_ecosystem/3_6_serde/request.yaml").unwrap();
        file.write_all(yaml_string.as_bytes()).unwrap();

        let mut file = File::open("../../3_ecosystem/3_6_serde/request.yaml").unwrap();
        let mut yaml_string = String::new();
        file.read_to_string(&mut yaml_string).unwrap();
        let deserialized_req: Request = serde_yaml::from_str(&yaml_string).unwrap();

        assert_eq!(original_req, deserialized_req);
    }
}
