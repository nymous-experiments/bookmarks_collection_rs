use chrono::serde::ts_microseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

/// ```json
/// {
///   "guid": "root________",
///   "title": "",
///   "index": 0,
///   "dateAdded": 1509553862576000,
///   "lastModified": 1563557348382000,
///   "id": 1,
///   "typeCode": 2,
///   "type": "text/x-moz-place-container",
///   "root": "placesRoot",
///   "children": []
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkRoot {
    guid: Guid,
    title: String,
    index: usize,
    #[serde(with = "ts_microseconds")]
    date_added: DateTime<Utc>,
    #[serde(with = "ts_microseconds")]
    last_modified: DateTime<Utc>,
    id: usize,
    type_code: usize, // TODO use Enum
    r#type: String,   // TODO?
    root: String,
    #[serde(default)]
    children: Vec<Bookmark>, // TODO
}

impl Explorable for BookmarkRoot {
    fn get_children(&self) -> &Vec<Bookmark> {
        &self.children
    }
}

impl Display for BookmarkRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Root: {}", self.title)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Bookmark {
    // Root(BookmarkRoot), // TODO: A root has the same type as a folder, how to distinguish between them?
    #[serde(rename = "text/x-moz-place-container")]
    Folder(BookmarkFolder),
    #[serde(rename = "text/x-moz-place")]
    Entry(BookmarkEntry),
    #[serde(rename = "text/x-moz-place-separator")]
    Separator(BookmarkSeparator),
}

/// ```json
/// {
///   "guid": "1GXuDpWKuzaZ",
///   "title": "Docs",
///   "index": 23,
///   "dateAdded": 1514400981910000,
///   "lastModified": 1563557349000000,
///   "id": 1330,
///   "typeCode": 2,
///   "type": "text/x-moz-place-container",
///   "children": []
/// }
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkFolder {
    guid: Guid,
    title: String,
    index: usize,
    #[serde(with = "ts_microseconds")]
    date_added: DateTime<Utc>,
    #[serde(with = "ts_microseconds")]
    last_modified: DateTime<Utc>,
    id: usize,
    type_code: usize, // TODO use Enum
    // r#type: String,          // TODO? Handled by the tagged Enum matching above?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    children: Vec<Bookmark>, // TODO
}

impl Explorable for BookmarkFolder {
    fn get_children(&self) -> &Vec<Bookmark> {
        &self.children
    }
}

impl Display for BookmarkFolder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Folder: {}", self.title)
    }
}

/// ```json
/// {
///   "guid": "ioFmy5yuXCAv",
///   "title": "",
///   "index": 16,
///   "dateAdded": 1497742450465000,
///   "lastModified": 1563557349000000,
///   "id": 343,
///   "typeCode": 3,
///   "type": "text/x-moz-place-separator"
/// }
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkSeparator {
    guid: Guid,
    title: String,
    index: usize,
    #[serde(with = "ts_microseconds")]
    date_added: DateTime<Utc>,
    #[serde(with = "ts_microseconds")]
    last_modified: DateTime<Utc>,
    id: usize,
    type_code: usize, // TODO use Enum
                      // r#type: String,   // TODO?
}

impl Display for BookmarkSeparator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Separator")
    }
}
/// ```json
/// {
///   "guid": "szjqsI0NdTuZ",
///   "title": "Bookmark title",
///   "index": 0,
///   "dateAdded": 1601467891225000,
///   "lastModified": 1601658040000000,
///   "id": 1234,
///   "typeCode": 1,
///   "charset": "UTF-8",
///   "iconuri": "https://assets-cdn.github.com/favicon.ico",
///   "type": "text/x-moz-place",
///   "uri": "https://example.com"
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkEntry {
    guid: Guid,
    title: String,
    index: usize,
    #[serde(with = "ts_microseconds")]
    date_added: DateTime<Utc>,
    #[serde(with = "ts_microseconds")]
    last_modified: DateTime<Utc>,
    id: usize,
    type_code: usize, // TODO use Enum
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Tags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_uri: Option<String>,
    // r#type: String,   // TODO?
    uri: String,
}

impl Display for BookmarkEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entry: {} ({})", self.title, self.uri)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Guid(String);

#[derive(Debug)]
pub struct Tags(HashSet<String>);

impl Serialize for Tags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // https://stackoverflow.com/a/47582249/3641865
        serializer.serialize_str(&itertools::join(&self.0, ","))
    }
}

impl<'de> Deserialize<'de> for Tags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // https://stackoverflow.com/a/54008190/3641865
        let str_sequence = String::deserialize(deserializer)?;
        Ok(Tags(
            str_sequence
                .split(',')
                .map(|item| item.to_owned())
                .collect(),
        ))
    }
}

/*impl Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Guid", &self.0)
    }
}

impl<'de> Deserialize<'de> for Guid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(String::deserialize(deserializer)?))
    }
}*/

pub trait Explorable {
    fn get_children(&self) -> &Vec<Bookmark>;
}

pub fn explore_bookmarks<T: Explorable + Display>(bookmark: &T) {
    println!("{bookmark}");
    for child in bookmark.get_children() {
        match child {
            Bookmark::Folder(f) => {
                println!("{f}");
                explore_bookmarks(f);
            }
            Bookmark::Entry(e) => {
                println!("{e}");
            }
            Bookmark::Separator(s) => {
                println!("{s}");
            }
        }
    }
}
