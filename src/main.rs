#[macro_use]
extern crate derive_builder;

use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

use xml::writer::{EmitterConfig, EventWriter, Result as XResult, XmlEvent};

#[derive(Builder, Debug)]
struct Track {
    url: String,
    title: String,
}

impl Track {
    fn new(url: String, title: String) -> Track {
        Track {
            url: url,
            title: title,
        }
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
}

fn main() {
    let file = File::open("VIDs").expect("Failed to open file!");
    let mut out = File::create("VIDs.xspf").unwrap();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut out);
    convert(
        tracks(
            BufReader::new(file)
                .lines()
                .map(|line| line.expect("Failed to read line")),
        ),
        &mut writer,
    )
    .expect("Failed to parse");
}

fn convert<I: Iterator<Item = Track>, W: Write>(tracks: I, w: &mut EventWriter<W>) -> XResult<()> {
    w.write(
        XmlEvent::start_element("playlist")
            .ns("", "http://xspf.org/ns/0/")
            .ns("vlc", "http://www.videolan.org/vlc/playlist/ns/0/"),
    )?;
    w.write(XmlEvent::start_element("trackList"))?;
    for track in tracks {
        track.write_xml(w)?;
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
                    builder.title(
                        line.chars()
                            .skip_while(|c| c != &'=')
                            .skip(1)
                            .collect::<String>(),
                    );
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
