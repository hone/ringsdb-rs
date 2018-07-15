extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use serde::de::Error;

#[derive(Clone, PartialEq)]
pub enum TypeCode {
    Hero,
    Ally,
    Event,
    Attachment,
    SideQuest,
    Treasure,
}

impl serde::Serialize for TypeCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
        {
            serializer.serialize_str(match *self {
                TypeCode::Hero       => "hero",
                TypeCode::Ally       => "ally",
                TypeCode::Event      => "event",
                TypeCode::Attachment => "attachment",
                TypeCode::SideQuest  => "player-side-quest",
                TypeCode::Treasure   => "treasure",
            })
        }
}

impl<'de> serde::Deserialize<'de> for TypeCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
        {
            let s = String::deserialize(deserializer)?;
            match s.as_str() {
                "hero"              => Ok(TypeCode::Hero),
                "ally"              => Ok(TypeCode::Ally),
                "event"             => Ok(TypeCode::Event),
                "attachment"        => Ok(TypeCode::Attachment),
                "player-side-quest" => Ok(TypeCode::SideQuest),
                "treasure"          => Ok(TypeCode::Treasure),
                string              => Err(D::Error::invalid_value(serde::de::Unexpected::Str(string), &"TypeCode")),
            }
        }
}

#[derive(Clone, PartialEq)]
pub enum TypeName {
    Hero,
    Ally,
    Event,
    Attachment,
    SideQuest,
    Treasure,
}

impl serde::Serialize for TypeName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
        {
            serializer.serialize_str(match *self {
                TypeName::Hero       => "Hero",
                TypeName::Ally       => "Ally",
                TypeName::Event      => "Event",
                TypeName::Attachment => "Attachment",
                TypeName::SideQuest  => "Player Side Quest",
                TypeName::Treasure   => "Treasure",
            })
        }
}

impl<'de> serde::Deserialize<'de> for TypeName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
        {
            let s = String::deserialize(deserializer)?;
            match s.as_str() {
                "Hero"              => Ok(TypeName::Hero),
                "Ally"              => Ok(TypeName::Ally),
                "Event"             => Ok(TypeName::Event),
                "Attachment"        => Ok(TypeName::Attachment),
                "Player Side Quest" => Ok(TypeName::SideQuest),
                "Treasure"          => Ok(TypeName::Treasure),
                string              => Err(D::Error::invalid_value(serde::de::Unexpected::Str(string), &"TypeName")),
            }
        }
}

#[derive(Clone, PartialEq)]
pub enum SphereCode {
    Leadership,
    Tactics,
    Spirit,
    Lore,
    Neutral,
    Baggins,
    Fellowship,
}

impl serde::Serialize for SphereCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
        {
            serializer.serialize_str(match *self {
                SphereCode::Leadership => "leadership",
                SphereCode::Tactics    => "tactics",
                SphereCode::Spirit     => "spirit",
                SphereCode::Lore       => "lore",
                SphereCode::Neutral    => "neutral",
                SphereCode::Baggins    => "baggins",
                SphereCode::Fellowship => "fellowship",
            })
        }
}

impl<'de> serde::Deserialize<'de> for SphereCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
        {
            let s = String::deserialize(deserializer)?;
            match s.as_str() {
                "leadership" => Ok(SphereCode::Leadership),
                "tactics"    => Ok(SphereCode::Tactics),
                "spirit"     => Ok(SphereCode::Spirit),
                "lore"       => Ok(SphereCode::Lore),
                "neutral"    => Ok(SphereCode::Neutral),
                "baggins"    => Ok(SphereCode::Baggins),
                "fellowship" => Ok(SphereCode::Fellowship),
                string       => Err(D::Error::invalid_value(serde::de::Unexpected::Str(string), &"SphereCode")),
            }
        }
}

#[derive(Clone, PartialEq)]
pub enum SphereName {
    Leadership,
    Tactics,
    Spirit,
    Lore,
    Neutral,
    Baggins,
    Fellowship,
}

impl serde::Serialize for SphereName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
        {
            serializer.serialize_str(match *self {
                SphereName::Leadership => "Leadership",
                SphereName::Tactics    => "Tactics",
                SphereName::Spirit     => "Spirit",
                SphereName::Lore       => "Lore",
                SphereName::Neutral    => "Neutral",
                SphereName::Baggins    => "Baggins",
                SphereName::Fellowship => "Fellowship",
            })
        }
}

impl<'de> serde::Deserialize<'de> for SphereName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
        {
            let s = String::deserialize(deserializer)?;
            match s.as_str() {
                "Leadership" => Ok(SphereName::Leadership),
                "Tactics"    => Ok(SphereName::Tactics),
                "Spirit"     => Ok(SphereName::Spirit),
                "Lore"       => Ok(SphereName::Lore),
                "Neutral"    => Ok(SphereName::Neutral),
                "Baggins"    => Ok(SphereName::Baggins),
                "Fellowship" => Ok(SphereName::Fellowship),
                string       => Err(D::Error::invalid_value(serde::de::Unexpected::Str(string), &"SphereName")),
            }
        }
}

#[derive(Clone, Deserialize)]
#[allow(dead_code)]
pub struct Card {
    pub pack_code: String,
    pub pack_name: String,
    pub type_code: TypeCode,
    pub type_name: TypeName,
    pub sphere_code: SphereCode,
    pub sphere_name: SphereName,
    pub position: usize,
    pub code: String,
    pub name: String,
    pub traits: Option<String>,
    pub text: String,
    pub flavor: Option<String>,
    pub is_unique: bool,
    pub threat: Option<usize>,
    pub willpower: Option<usize>,
    pub attack: Option<usize>,
    pub defense: Option<usize>,
    pub health: Option<usize>,
    pub quantity: usize,
    pub deck_limit: usize,
    pub illustrator: String,
    pub octgnid: String,
    pub has_errata: bool,
    pub url: String,
    pub imagesrc: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Pack {
    pub name: String,
    pub code: String,
    pub position: usize,
    pub cycle_position: usize,
    pub available: String,
    pub known: usize,
    pub total: usize,
    pub url: String,
    pub id: usize,
}

const API_BASE_URL: &'static str = "https://ringsdb.com/api";

pub struct RingsDB {
    client: reqwest::Client,
}

impl RingsDB {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    #[allow(dead_code)]
    pub fn fetch_card(&self, card_code: &str) -> Result<Card, reqwest::Error> {
        let res = self.client.get(&format!("{}/public/card/{}", API_BASE_URL, card_code)).send()?
            .error_for_status();

        Ok(res?.json()?)
    }

    #[allow(dead_code)]
    pub fn fetch_cards(&self, pack_code: Option<&str>) -> Result<Vec<Card>, reqwest::Error> {
        let code = pack_code.unwrap_or("");
        let res = self.client.get(&format!("{}/public/cards/{}", API_BASE_URL, code)).send()?
            .error_for_status();

        Ok(res?.json()?)
    }

    #[allow(dead_code)]
    pub fn fetch_packs(&self) -> Result<Vec<Pack>, reqwest::Error> {
        let res = self.client.get(&format!("{}/public/packs/", API_BASE_URL)).send()?
            .error_for_status();

        Ok(res?.json()?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
