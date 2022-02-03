use crate::config;

extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::fs;

use xml::reader::{EventReader, XmlEvent};

use raster::Image;
use raster::{editor, PositionMode};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

struct PlistTexture
{
    name: String,
    texture_rect: Vec<i32>,
}

pub fn read_plist_file()
{
    let file_url = format!("{}/Resources/GJ_GameSheet.plist", config::get_installation_path());
    println!("{}", file_url);
    let file = File::open(file_url).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;

    let mut is_in_frames : bool = false;
    let mut is_inside_dict : bool = false;
    let mut is_texture_list_ready : bool = false;

    let mut get_texture_name : bool = false;
    let mut get_texture_rect : bool = false;

    let mut all_textures : Vec<PlistTexture> = Vec::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if is_in_frames && is_texture_list_ready
                {
                    if name.to_string() == "key" && !is_inside_dict
                    {
                        let mut p_tex = PlistTexture { name : "".to_string(), texture_rect: Vec::new() };
                        all_textures.push(p_tex);
                        get_texture_name = true;
                    }
                    else if name.to_string() == "dict"
                    {
                        is_inside_dict = true;
                    }
                }
                else if is_in_frames && name.to_string() == "dict" 
                {
                    is_texture_list_ready = true;
                }
            }

            Ok(XmlEvent::Characters(text)) => {
                if text.to_string() == "frames" { is_in_frames = true; }

                if get_texture_name 
                {
                    if text.to_string() == "metadata" { all_textures.pop(); break; }

                    let last_n = all_textures.len() - 1;
                    all_textures[last_n].name = text.clone(); 
                    get_texture_name = false;
                }

                if get_texture_rect
                {
                    let last_n = all_textures.len() - 1;
                    all_textures[last_n].texture_rect = text_to_vector4(text.clone());
                    /*println!("Texture Rect: [x: {}, y: {}, width: {}, height: {}]", 
                        all_textures[last_n].texture_rect[0],
                        all_textures[last_n].texture_rect[1],
                        all_textures[last_n].texture_rect[2],
                        all_textures[last_n].texture_rect[3]);*/

                    get_texture_rect = false;
                }

                if text.to_string() == "textureRect" { get_texture_rect = true; }
            }

            Ok(XmlEvent::EndElement { name }) => {
                if is_inside_dict && name.to_string() == "dict" { is_inside_dict = false; }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    let game_path = format!(r#"{}\Game"#, config::get_installation_path());
    let textures_path = format!(r#"{}\Game\Textures"#, config::get_installation_path());
    println!("Texture Path: {}", textures_path);
    if !config::exists(game_path.as_str()) { fs::create_dir(game_path).expect("Oops"); }
    if !config::exists(textures_path.as_str()) { fs::create_dir(textures_path).expect("Oops"); }

    let mut r_image = raster::open(format!("{}/Resources/GJ_GameSheet.png", config::get_installation_path()).as_str()).unwrap();

    println!("Starting Texture Unpacker...");

    for i in 0..all_textures.len()
    {
        save_plist_texture(all_textures[i].name.clone(), 
            r_image.clone(), 
            all_textures[i].texture_rect[0], 
            all_textures[i].texture_rect[1], 
            all_textures[i].texture_rect[2], 
            all_textures[i].texture_rect[3]);
    }
}

pub fn text_to_vector4(text: String) -> Vec<i32>
{
    let fuck = text.replace("{", "");
    let you = fuck.replace("}", "");
    let mut value_split : Vec<&str> = you.split(',').collect();

    let mut result : Vec<i32> = Vec::new();
    result.push(value_split[0].parse::<i32>().unwrap());
    result.push(value_split[1].parse::<i32>().unwrap());
    result.push(value_split[2].parse::<i32>().unwrap());
    result.push(value_split[3].parse::<i32>().unwrap());

    return result;
}

pub fn save_plist_texture(name: String, mut img: Image, x: i32, y: i32, width: i32, height: i32)
{
    println!("Saving {}...", format!("{}/Game/Textures/{}", config::get_installation_path(), name));
    editor::crop(&mut img, width, height, PositionMode::TopLeft, x, y).unwrap();
    raster::save(&img, format!("{}/Game/Textures/{}", config::get_installation_path(), name).as_str()).unwrap();
}