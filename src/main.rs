#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod conf;
mod opts;

use conf::*;
use dirs;
use opts::*;
use std::env;
use std::fs::File;
use std::io::{self, Read, Stdin, Write};
use std::io::{BufRead, BufReader};
use std::process::exit;
use xml::writer::{EmitterConfig, EventWriter, Result as XResult, XmlEvent};

#[derive(Builder, Debug)]
struct Track {
    url: String,
    title: String,
}

impl Track {
    fn new(url: String, title: String) -> Track {
        Track { url: url, title: title }
    }
    fn write_xml<W: Write>(self, w: &mut EventWriter<W>) -> XResult<()> {
        w.write(XmlEvent::start_element("track"))?;
        w.write(XmlEvent::start_element("location"))?;
        w.write(XmlEvent::characters(&self.url))?;
        w.write(XmlEvent::end_element())?;
        w.write(XmlEvent::start_element("title"))?;
        w.write(XmlEvent::characters(&self.title))?;
        w.write(XmlEvent::end_element())?;
        w.write(XmlEvent::end_element())
    }

    fn write_html<W: Write>(self, w: &mut EventWriter<W>) -> XResult<()> {
        w.write(XmlEvent::start_element("p"))?;
        w.write(XmlEvent::characters(&self.title))?;
        w.write(XmlEvent::end_element())?;
        w.write(
            XmlEvent::start_element("video")
                .attr("controls", "controls")
                .attr("preload", "none"),
        )?;
        w.write(XmlEvent::start_element("source").attr("src", &self.url))?;
        w.write(XmlEvent::end_element())?;
        w.write(XmlEvent::end_element())
    }
}

lazy_static! {
    static ref CONFIG: Config = config().unwrap_or(Config::default());
}

fn config() -> Option<Config> {
    match dirs::config_dir() {
        None => {
            eprintln!("Couldn't locate configuration directory, proceeding with default config");
            None
        }
        Some(dir) => {
            let mut file_path = dir;
            file_path.push("aria2xspf.toml");
            if !file_path.exists() {
                return None;
            }
            let mut file = File::open(file_path).expect("Failed to load config file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to decode config");
            Some(toml::from_str(&contents).expect("Failed to parse config"))
        }
    }
}

fn main() {
    let opts = Opts::from_args();
    if opts.input == opts.output {
        eprintln!("Input and output are the same file! {:?}\nContinue(y/n): ", opts.input);
        match io::stdin().bytes().next() {
            Some(Ok(r)) if r as char == 'y' => (),
            Some(Ok(_)) => exit(1),
            _ => (), //Stdin closed, assume in == out is intentional
        }
    }
    let file = File::open(opts.input).expect("Failed to open file!");
    let mut out = File::create(opts.output).expect("Failed to open OUTPUT file!");
    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(&mut out);
    convert(
        opts.html,
        tracks(
            BufReader::new(file)
                .lines()
                .map(|line| line.expect("Failed to read line")),
        ),
        &mut writer,
    )
    .expect("Failed to parse");
}

fn convert<I: Iterator<Item = Track>, W: Write>(html: bool, tracks: I, w: &mut EventWriter<W>) -> XResult<()> {
    if html {
        w.write(XmlEvent::start_element("html"))?;
        {
            w.write(XmlEvent::start_element("head"))?;
            for url in CONFIG.include.js.iter() {
                w.write(
                    XmlEvent::start_element("script")
                        .attr("type", "text/javascript")
                        .attr("src", url),
                )?;
                w.write(XmlEvent::characters(""))?;
                w.write(XmlEvent::end_element())?;
            }
            for url in CONFIG.include.css.iter() {
                w.write(
                    XmlEvent::start_element("link")
                        .attr("rel", "stylesheet")
                        .attr("href", url),
                )?;
                w.write(XmlEvent::characters(""))?;
                w.write(XmlEvent::end_element())?;
            }
            w.write(XmlEvent::end_element())?;
        }
        w.write(XmlEvent::start_element("body"))?;
    } else {
        w.write(
            XmlEvent::start_element("playlist")
                .ns("", "http://xspf.org/ns/0/")
                .ns("vlc", "http://www.videolan.org/vlc/playlist/ns/0/"),
        )?;
        w.write(XmlEvent::start_element("trackList"))?;
    }
    for track in tracks {
        if html {
            track.write_html(w)?;
        } else {
            track.write_xml(w)?;
        }
    }
    w.write(XmlEvent::end_element())?;
    w.write(XmlEvent::end_element())
}

fn tracks<'a, I: Iterator<Item = String> + 'a>(aria: I) -> Box<Iterator<Item = Track> + 'a> {
    let tracks = aria
        .scan(TrackBuilder::default(), |builder, line| {
            if !builder.url.is_some() {
                builder.url(line);
            } else {
                if line.starts_with("\tout=") {
                    builder.title(line.chars().skip_while(|c| c != &'=').skip(1).collect::<String>());
                    let track = Some(builder.build().unwrap());
                    *builder = TrackBuilder::default();
                    return Some(track);
                }
            }
            Some(None)
        })
        .filter_map(|track| track);
    Box::new(tracks)
}
