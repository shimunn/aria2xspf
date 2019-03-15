#[macro_use]
extern crate derive_builder;

use std::io::{self, Write};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use xml::writer::{EventWriter, EmitterConfig, XmlEvent, Result as XResult};

#[derive(Builder, Debug)]
struct Track {
 url: String,
 title: String
}

impl Track {
 
 fn new(url: String, title: String) -> Track {
  Track{
   url: url,
   title: title
  }
 }
 fn write_xml<W: Write>(self,w: &mut EventWriter<W>) -> XResult<()> {
  let events: [XmlEvent; 8] = [
   XmlEvent::start_element("track".into()).into(),
   XmlEvent::start_element("location".into()).into(),
   XmlEvent::characters(&self.url).into(),
   XmlEvent::end_element().into(),
   XmlEvent::start_element("title".into()).into(),
   XmlEvent::characters(&self.title).into(),
   XmlEvent::end_element().into(),
   XmlEvent::end_element().into()
  ];
  for event in &events {
   w.write(event.into()).into()?;
  }
  Ok(())
 }

}

fn main() {
    let file = File::open("VIDs").expect("Failed to open file!");
    let mut out = File::create("VIDs.xspf").unwrap();
    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(&mut out);
    convert(tracks(BufReader::new(file).lines().map(|line| line.expect("Failed to read line"))), &mut writer).expect("Failed to parse");
}

fn convert<I: Iterator<Item=Track>, W: Write>(tracks: I,w:  &mut EventWriter<W>) -> XResult<()> {
 w.write(XmlEvent::start_element("playlist".into()).ns("", "http://xspf.org/ns/0/").ns("vlc", "http://www.videolan.org/vlc/playlist/ns/0/")).into()?;
 w.write(XmlEvent::start_element("trackList".into())).into()?;
 for track in tracks {
  track.write_xml(w)?;
 }
 w.write(XmlEvent::end_element().into()).into()?;
 w.write(XmlEvent::end_element().into()).into()?
}

fn tracks<'a, I: Iterator<Item=String> + 'a>(aria: I) -> Box<Iterator<Item=Track> + 'a> {
 let tracks = aria.scan(TrackBuilder::default(), |mut builder, line| {
  if !builder.url.is_some() {
   builder.url(line);
  } else {
   if line.starts_with("\t") {
    builder.title(line.chars().skip_while(|c| c != &'=').skip(1).collect::<String>());
    let track = Some(builder.build().unwrap());
    *builder = TrackBuilder::default();
    return Some(track);
   }
  }
  Some(None)
 }).filter_map(|track| track);
 Box::new(tracks)
}
