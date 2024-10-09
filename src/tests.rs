extern crate regex;

use super::{version as ver, Flags, Magic};

#[test]
fn version() {
    let re = regex::Regex::new("\\d+.\\d+.\\d+").unwrap();
    assert!(re.is_match(ver()));
}

#[test]
fn load_default_db() {
    let cookie = Magic::open(Flags::NONE | Flags::ERROR).unwrap();
    assert!(cookie.load::<String>(&[]).is_ok());
}

#[test]
fn load_one_db() {
    let cookie = Magic::open(Flags::NONE | Flags::ERROR).unwrap();
    assert!(cookie.load(&vec!["data/db-images-png"]).is_ok());
}

#[test]
fn get_file_mime() {
    let cookie = Magic::open(Flags::NONE).unwrap();
    assert!(cookie.load(&vec!["data/db-images-png"]).is_ok());

    let path = "data/rust-logo-128x128-blk.png";

    assert_eq!(
        cookie.file(&path).unwrap(),
        "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced"
    );

    cookie.set_flags(Flags::MIME_TYPE);
    assert_eq!(cookie.file(&path).unwrap(), "image/png");

    cookie.set_flags(Flags::MIME_TYPE | Flags::MIME_ENCODING);
    assert_eq!(cookie.file(&path).unwrap(), "image/png; charset=binary");
}

#[test]
fn get_buffer_mime() {
    let cookie = Magic::open(Flags::NONE).unwrap();
    assert!(cookie.load(&vec!["data/db-python"].as_slice()).is_ok());

    let s = b"#!/usr/bin/env python\nprint('Hello, world!')";
    assert_eq!(
        cookie.buffer(s).unwrap(),
        "Python script, ASCII text executable"
    );

    cookie.set_flags(Flags::MIME_TYPE);
    assert_eq!(cookie.buffer(s).unwrap(), "text/x-python");
}

#[test]
fn file_error() {
    let cookie = Magic::open(Flags::NONE | Flags::ERROR).unwrap();
    assert!(cookie.load::<String>(&[]).is_ok());

    let ret = cookie.file("non-existent_file.txt");
    assert!(ret.is_err());
    assert_eq!(
        ret.err().unwrap().desc,
        "cannot stat `non-existent_file.txt' (No such file or directory)"
    );
}

#[test]
fn macro_load_default_db() {
    assert!(magic!().is_ok());
}

#[test]
fn macro_load_one_db() {
    assert!(magic!(,&vec!["data/db-images-png"]).is_ok());
}

#[test]
fn macro_load_one_db_with_flags() {
    assert!(magic!(Flags::NONE | Flags::ERROR, &vec!["data/db-images-png"]).is_ok());
}

#[test]
fn macro_get_file_mime() {
    let cookie = magic!().unwrap();
    let path = "data/rust-logo-128x128-blk.png";

    assert_eq!(
        cookie.file(&path).unwrap(),
        "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced"
    );

    cookie.set_flags(Flags::MIME_TYPE);
    assert_eq!(cookie.file(&path).unwrap(), "image/png");

    cookie.set_flags(Flags::MIME_TYPE | Flags::MIME_ENCODING);
    assert_eq!(cookie.file(&path).unwrap(), "image/png; charset=binary");
}

#[test]
fn macro_get_buffer_mime() {
    let cookie = magic!().unwrap();
    assert!(cookie.load(&vec!["data/db-python"].as_slice()).is_ok());

    let s = b"#!/usr/bin/env python\nprint('Hello, world!')";
    assert_eq!(
        cookie.buffer(s).unwrap(),
        "Python script, ASCII text executable"
    );

    cookie.set_flags(Flags::MIME_TYPE);
    assert_eq!(cookie.buffer(s).unwrap(), "text/x-python");
}
