use anyhow::Result;
use log::debug;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Dungeon {
    pub name: String,
    pub tanks: u8,
    pub healers: u8,
    pub strikers: u8,
}

pub fn read_config() -> Result<Vec<Dungeon>> {
    debug!("Reading config");
    let from_config = fs::read_to_string("dungeons.yaml")?;
    let dungeons = serde_yaml::from_str(&from_config)?;
    debug!("{:#?}", dungeons);
    Ok(dungeons)
}

#[cfg(test)]
mod tests {
    use super::Dungeon;

    #[test]
    fn deserialize_config() {
        let s = r#"---
        - name: Dungeon 1
          tanks: 1
          healers: 1
          strikers: 2
        - name: Dungeon 2
          tanks: 2
          healers: 2
          strikers: 4
        "#;
        let structs: Vec<Dungeon> = serde_yaml::from_str(s).unwrap();
        assert_eq!(structs.len(), 2);
    }
}
