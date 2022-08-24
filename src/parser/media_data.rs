use std::collections::HashMap;
use std::fmt;
use crate::parser::parse::Parse;

use anyhow::Result;
use crate::parser::{headers, Parser};

#[derive(Debug)]
pub struct MediaData {
    ttype: String,
    group_id: String,
    name: String,
    language: String,
    default: String,
    autoselect: String,
    channels: String,
    uri: String
}

impl Parse for MediaData {
    fn from_string(contents: String) -> Result<Box<Self>> {
        let mut fields: HashMap<String, String> = HashMap::new();
        let cleaned_contents = contents.replace(&headers::MEDIA_DATA, "");
        let iter_contents = cleaned_contents.split(',').collect::<Vec<&str>>();
        for c in iter_contents {
            let pair = c.split('=').collect::<Vec<_>>();
            fields.insert(pair[0].to_string(), pair[1].to_string());
        };
        Self::from_hashmap(fields)
    }

    fn from_hashmap(map: HashMap<String, String>) -> Result<Box<Self>> {
        Ok(
            Box::new(
                Self {
                    ttype: Parser::parse_to_string("TYPE", &map)?,
                    group_id: Parser::parse_to_string("GROUP-ID", &map)?,
                    name: Parser::parse_to_string("NAME", &map)?,
                    language: Parser::parse_to_string("LANGUAGE", &map)?,
                    default: Parser::parse_to_string("DEFAULT", &map)?,
                    autoselect: Parser::parse_to_string("AUTOSELECT", &map)?,
                    channels: Parser::parse_to_string("CHANNELS", &map)?,
                    uri: Parser::parse_to_string("URI", &map)?
                }
            )
        )
    }
}

impl fmt::Display for MediaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\
            TYPE={},\
            GROUP-ID={},\
            NAME={},\
            LANGUAGE={},\
            DEFAULT={},\
            AUTOSELECT={},\
            CHANNELS={},\
            URI={}\
            ", self.ttype, self.group_id, self.name, self.language, self.default, self.autoselect, self.channels, self.uri)
    }
}
