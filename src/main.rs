use assert_json_diff::assert_json_eq;
use bookmarks_collection::{Bookmark, BookmarkFolder, BookmarkRoot, Explorable};
use serde_json::Value;
use std::fs::{read_to_string, File};
use std::io;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let json = r#"{
      "guid": "szjqsI0NdTuZ",
      "title": "Bookmark title",
      "index": 0,
      "dateAdded": 1601467891225000,
      "lastModified": 1601658040000000,
      "id": 1234,
      "typeCode": 1,
      "type": "text/x-moz-place",
      "uri": "https://example.com"
    }"#;

    println!("Simple JSON");
    let bookmark: Bookmark = serde_json::from_str(json).unwrap();
    println!("deserialized = {bookmark:#?}");

    let serialized = serde_json::to_string_pretty(&bookmark).unwrap();
    println!("serialized = {serialized}");

    assert_json_eq!(
        serde_json::from_str::<Value>(&serialized).unwrap(),
        serde_json::from_str::<Value>(json).unwrap()
    );

    println!("JSON through Enum");
    let unknown_bookmark: Bookmark = serde_json::from_str(json).unwrap();
    println!("deserialized = {unknown_bookmark:#?}");

    println!("Big JSON with everything");
    let big_json = r#"{
      "guid": "root________",
      "title": "",
      "index": 0,
      "dateAdded": 1509553862576000,
      "lastModified": 1563557348382000,
      "id": 1,
      "typeCode": 2,
      "type": "text/x-moz-place-container",
      "root": "placesRoot",
      "children": [
        {
          "guid": "1GXuDpWKuzaZ",
          "title": "Docs",
          "index": 23,
          "dateAdded": 1514400981910000,
          "lastModified": 1563557349000000,
          "id": 1330,
          "typeCode": 2,
          "type": "text/x-moz-place-container",
          "children": [
            {
              "guid": "irnkVN3Z0Wm8",
              "title": "facebook/Docusaurus: Easy to maintain open source documentation websites.",
              "index": 0,
              "dateAdded": 1514400959484000,
              "lastModified": 1563557349000000,
              "id": 1329,
              "typeCode": 1,
              "charset": "UTF-8",
              "iconUri": "https://assets-cdn.github.com/favicon.ico",
              "type": "text/x-moz-place",
              "uri": "https://github.com/facebook/Docusaurus"
            },
            {
              "guid": "ioFmy5yuXCAv",
              "title": "",
              "index": 16,
              "dateAdded": 1497742450465000,
              "lastModified": 1563557349000000,
              "id": 343,
              "typeCode": 3,
              "type": "text/x-moz-place-separator"
            },
            {
              "guid": "1GXuDpWKuzaZ",
              "title": "Folder 2",
              "index": 23,
              "dateAdded": 1514400981910000,
              "lastModified": 1563557349000000,
              "id": 1330,
              "typeCode": 2,
              "type": "text/x-moz-place-container",
              "children": [
                {
                  "guid": "szjqsI0NdTuZ",
                  "title": "Common security issues in financially-orientated web",
                  "index": 0,
                  "dateAdded": 1601467891225000,
                  "lastModified": 1601658040000000,
                  "id": 3981,
                  "typeCode": 1,
                  "type": "text/x-moz-place",
                  "uri": "https://www.nccgroup.trust/globalassets/our-research/uk/images/common_security_issues_in_financially-orientated_web.pdf.pdf"
                }
              ]
            }
          ]
        }
      ]
    }"#;
    let bookmarks: BookmarkRoot = serde_json::from_str(big_json).unwrap();
    println!("{bookmarks:#?}");

    let serialized = serde_json::to_string_pretty(&bookmarks).unwrap();
    println!("serialized = {serialized}");

    assert_json_eq!(
        serde_json::from_str::<Value>(&serialized).unwrap(),
        serde_json::from_str::<Value>(big_json).unwrap()
    );

    println!("{}", bookmarks);
    bookmarks_collection::explore_bookmarks(&bookmarks);

    println!("JSON from file");
    // let all_bookmarks_json = include_str!("../bookmarks.json");
    let all_bookmarks_json = File::open("bookmarks.json")?;
    let buf_reader = BufReader::new(all_bookmarks_json);
    let all_bookmarks: Bookmark = serde_json::from_reader(buf_reader)?;
    // let all_bookmarks: Bookmark = serde_json::from_str(all_bookmarks_json)?;
    println!();
    println!();
    println!();

    // println!("{all_bookmarks:#?}");
    if let Bookmark::Folder(f) = &all_bookmarks {
        if let Bookmark::Folder(f) = &f.get_children().get(10).unwrap() {
            if let Bookmark::Entry(e) = &f.get_children().get(120).unwrap() {
                println!("{:#?}", e);
                println!("{}", serde_json::to_string_pretty(&e).unwrap());
            }
        }
    }
    let serialized = serde_json::to_string_pretty(&all_bookmarks).unwrap();
    assert_json_eq!(
        serde_json::from_str::<Value>(&serialized).unwrap(),
        serde_json::from_str::<Value>(&read_to_string("bookmarks.json")?).unwrap()
    );

    Ok(())
}
