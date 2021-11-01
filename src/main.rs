#[macro_use]
extern crate rss;

use clap::{AppSettings, Parser};
use rss::ChannelBuilder;
use rss::ItemBuilder;
use rss::Channel;
use std;
use std::fs::File;
use std::io::{self, Write, BufReader};
use chrono::{Utc};

#[derive(Parser)]
#[clap(version = "1.0", author="iranika <@happy_packet>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "mo4koma")]
    site: String,
    #[clap(short, long, default_value = "feed.xml")]
    outfile: String,
}

fn main() {

    let opts: Opts = Opts::parse();
 
    match opts.site.as_str() {
        "mo4koma" => mo4komaData(opts),
        _ => println!("Unknown site.")
    }
}

fn mo4komaData(opts: Opts){
    let isExist = std::path::Path::new(opts.outfile.as_str()).exists();
    let mut channel : rss::Channel;
    let mut file : std::fs::File;
    if (isExist){
        println!("file is exist.");
        file = File::open(opts.outfile).unwrap();
        channel = Channel::read_from(BufReader::new(&file)).unwrap();
    }else{
        channel = ChannelBuilder::default()
            .title("みちくさびゅあー")
            .link("https://movue.iranika.info")
            .description("ばっくやーど数こまの更新RSS")
            .build();
        file = File::create(opts.outfile.as_str()).unwrap();
    }
    let mut item = rss::Item::default();
    item.set_title("もういちど".to_string());
    item.set_link("https://movue.iranika.info/#/?page=latest".to_string());
    item.set_pub_date(Utc::now().to_rfc2822());
    channel.set_language("ja-JP".to_string());
    channel.set_items(vec![item]);
    //channel.set_items(item);
    write!(&file, "{}", channel.to_string());

}