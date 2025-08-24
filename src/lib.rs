use std::fs::File;
use std::io::prelude::*;
use reqwest::Response;

pub fn run(filename :&str) {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    //print_nippo_start();
    let res = post_slack(contents);
    println!("{:#?}", res.unwrap());
}

#[tokio::main]
async fn post_slack(post_message :String) -> Result<Response, Box<dyn std::error::Error>> {
    println!("With text:\n{}", post_message);

    let url = "https://slack.com/api/chat.postMessage";
    let token = "";
    let channel = "";
    let text = post_message;

    let params = [
        ("url", url),
        ("token", token),
        ("channel", channel),
        ("text", &text),
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

fn construct_mention_name(name: &str) -> Result<String, String>{
    Ok('@'.to_string() + name)
}

fn construct_mention_raw(names: &[&str]) -> Result<String, String> {
    let mut return_raw_string = String::new();
    for name in names.iter() {
        if return_raw_string.len() != 0 {
            return_raw_string.push(' ');
        }
        return_raw_string.push_str(&construct_mention_name(name).unwrap());
    }
    Ok(return_raw_string)
}

#[test]
fn construct_mention_name_basic() {
    assert_eq!(construct_mention_name("abc").unwrap(), "@abc");
    assert_eq!(construct_mention_name("試験山").unwrap(), "@試験山");
    assert_eq!(construct_mention_name("123").unwrap(), "@123");
    assert_eq!(construct_mention_name("").unwrap(), "@");
}

#[test]
fn construct_mention_raw_single() {
    assert_eq!(construct_mention_raw(&["abc"]).unwrap(), "@abc");
    assert_eq!(construct_mention_raw(&[""]).unwrap(), "@");
}

#[test]
fn construct_mention_raw_multiple() {
    assert_eq!(construct_mention_raw(&["abc", "cbd"]).unwrap(), "@abc @cbd");
    assert_eq!(construct_mention_raw(&["試験山", "abc", "cbd"]).unwrap(), "@試験山 @abc @cbd");
    assert_eq!(construct_mention_raw(&["a", "b", "c", "d"]).unwrap(), "@a @b @c @d");
}

#[test]
fn construct_mention_raw_empty() {
    assert_eq!(construct_mention_raw(&[]).unwrap(), "");
}

#[test]
fn construct_mention_name_unicode() {
    assert_eq!(construct_mention_name("山田太郎").unwrap(), "@山田太郎");
    assert_eq!(construct_mention_name("😀").unwrap(), "@😀");
}
