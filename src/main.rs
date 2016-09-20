/**
* Copyright (c) 2016 James Carroll
* main.rs is part of RAW Fire.
*
* RAW Fire is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* RAW Fire is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with RAW Fire.  If not, see <http://www.gnu.org/licenses/>.
*/

#[macro_use]
extern crate clap;

mod image;

use clap::{App, Arg};
use image::{IO, Config};

fn main() {
    let matches = App::new("RAW Fire")
        .about("RAW Image processing program that uses ArrayFire for GPU acceleration.")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("INPUT")
            .short("i")
            .long("input")
            .value_name("FILE")
            .takes_value(true)
            .required(true)
            .help("The full path to the image file to open."))
        .arg(Arg::with_name("OUTPUT PATH")
            .short("p")
            .long("out_path")
            .value_name("FILE")
            .takes_value(true)
            .required(false)
            .help("The path to export the image to. \nDEFAULT: same input path \
            with an added '_out' added to image name."))
        .arg(Arg::with_name("OUTPUT TYPE")
            .short("t")
            .long("out_type")
            .value_name("IMAGE TYPE")
            .takes_value(true)
            .required(false)
            .help("The image type to export to. \nDEFAULT: same as input image, \
            unless input is RAW then it's JPEG"))
        .arg(Arg::with_name("VERBOSE")
            .short("v")
            .long("verbose")
            .value_name("INT")
            .takes_value(true)
            .default_value("1")
            .required(false)
            .help("How much text such be printed while running: \n0 -- Off\n1 -- On, but only important things\
            2 -- On, print everything")
        )
        .arg(Arg::with_name("GUI")
            .short("g")
            .long("gui")
            .value_name("BOOL")
            .takes_value(true)
            .default_value("true")
            .required(false)
            .help("Show GUI: true / false"))
        .get_matches();

    let mut in_split_path: Vec<&str> = matches.value_of("INPUT").unwrap().split(".").collect();
    let in_type: String = if let Some(t) = in_split_path.pop() { t.to_string() } else { "jpeg".to_string()};
    let in_path: String = in_split_path.join(".").to_string();

    let conf = Config {
        path: if let Some(o) = matches.value_of("OUTPUT PATH") {
            IO::new(in_path, o.to_string())
        } else {
            IO::new(in_path.clone(), in_path + "_out")
        },
        file_type: if let Some(o) = matches.value_of("OUTPUT TYPE") {
            IO::new(in_type, o.to_string())
        } else {
            IO::new(in_type.clone(), in_type)
        },
        verbose: match matches.value_of("VERBOSE") {
            Some("0") => 0,
            Some("2") => 2,
            Some("1") | _ => 1,
        },
        gui: match matches.value_of("GUI") {
            Some("false") | Some("0") => false,
            Some("true") | Some("1") | _ => true,
        },
    };

    println!("Config: {:?}", conf);

}
