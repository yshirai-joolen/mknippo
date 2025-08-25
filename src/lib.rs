use std::fs::File;
use std::io::prelude::*;
use reqwest::Response;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    slack: SlackConfig,
}

#[derive(Deserialize)]
struct SlackConfig {
    url: String,
    token: String,
    channel: String,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn run(filename :&str) {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    //print_nippo_start();
    let res = post_slack(contents);
    println!("{:#?}", res.unwrap());
    println!("{:#?}", contents);
}

#[tokio::main]
async fn post_slack(post_message :String) -> Result<Response, Box<dyn std::error::Error>> {
    println!("With text:\n{}", post_message);

    let config = load_config()?;
    let url = &config.slack.url;
    let token = &config.slack.token;
    let channel = &config.slack.channel;
    let text = &post_message;

    let params = [
        ("url", url),
        ("token", token),
        ("channel", channel),
        ("text", text),
    ];

    let client = reqwest::Client::new();
    let resp = client.post(url)
        .form(&params)
        .send()
        .await?;
    Ok(resp)
}

pub fn print_nippo_start() {
    println!("{}", construct_mention_raw(&["山", "島", "swamp", "平地"]).unwrap());
    println!("■本日の予定■");
    println!("（2021/10/27）11:30～20:00勤務予定");
    println!("◇APOCALYPSE");
    println!("・ APOCALYPSE-5 レビュー集計 [次回11/1]");
    println!("・ APOCALYPSE-12 タグ設置 [顧客確認中]");
    println!("・ ローカル環境構築[作業中] :triplets_parrot:");
}

pub fn construct_mention_name(name: &str) -> Result<String, String>{
    Ok('@'.to_string() + name)
}

pub fn construct_mention_raw(names: &[&str]) -> Result<String, String> {
    let mut return_raw_string = String::new();
    for name in names.iter() {
        if return_raw_string.len() != 0 {
            return_raw_string.push(' ');
        // filepath: /Users/y-shirai/mknippo/tests/mention_tests.rs
        }
        return_raw_string.push_str(&construct_mention_name(name).unwrap());
    }
    Ok(return_raw_string)
}

