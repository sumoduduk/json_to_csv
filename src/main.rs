mod utils;

use serde::{Deserialize, Serialize};
use std::{env, fs::File};

use crate::utils::get_category_and_type;

#[derive(Debug, Deserialize)]
struct Image {
    filename: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Data {
    thumb: Image,
    image: Image,
    width: u32,
    height: u32,
    time: u32,
    title: String,
}

#[derive(Debug, Serialize)]
struct Record {
    id: u16,
    width: u32,
    height: u32,
    time: u32,
    title: String,
    filename: String,
    thumb_image: String,
    image: String,
    category: String,
    photo_type: String,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path_str = args
        .get(1)
        .expect("provide path location to your json in args 1");
    let out_path = args
        .get(2)
        .expect("provide output path to save your csv in args 2");

    let json_file = File::open(path_str).expect("cant open json file, make sure the file exist");
    let json_content: Vec<Data> =
        serde_json::from_reader(json_file).expect("your file format are not json format");

    let file_output = File::create(out_path).expect("cant create file named");

    let mut writer = csv::Writer::from_writer(file_output);
    let mut i: u16 = 1;

    for json_data in json_content {
        let (title, category, type_photo) = get_category_and_type(&json_data.title);

        let record = Record {
            id: i,
            width: json_data.width,
            height: json_data.height,
            time: json_data.time,
            title: title.to_string(),
            filename: json_data.image.filename,
            thumb_image: json_data.thumb.url,
            image: json_data.image.url,
            category: category.to_string(),
            photo_type: type_photo.to_string(),
        };

        println!("processing : {}", record.title);

        writer.serialize(record)?;
        i += 1;
    }

    writer.flush()?;

    Ok(())
}
