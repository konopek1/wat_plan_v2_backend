use reqwest;
use scraper::{Html, Selector};
use std::prelude::v1::Vec;
use tokio::fs::File;
use tokio::prelude::*;
use std::result::Result;
use std::thread;

const COOLDOWN: std::time::Duration = std::time::Duration::from_secs(7);
const URL: &str = "https://s1.wcy.wat.edu.pl/ed1/";
const FOLDER_GROUP: &str = "groups/";
const VMAX: usize = 22;
const HMAX: usize = 49;


#[derive(serde::Serialize,Clone)]
pub struct Krotka {
    title: String,
    class: String
}

impl Krotka {
    fn new(title: String,class: String) -> Krotka {
        return Krotka { title,class };
    }
}
type Task = tokio::task::JoinHandle<Result<(), std::io::Error>>;

#[tokio::main]
pub async fn fetch_parse_plan() -> Result<(), reqwest::Error> {
    let client: reqwest::Client = build_client().unwrap();

    let sid = get_sid(&client, URL).await?;
    println!("sid:{}", sid);

    let user_id = std::env::var("USER").expect("ERROR: UserId global var not set");
    let password = std::env::var("PASSWORD").expect("ERROR: Password global var not set");

    login(&client, &sid, user_id, password).await?;

    let groups = extract_groups(&sid).await;

    let mut tasks: Vec<Task> = Vec::new();

    for group in groups {
        let sido = sid.clone();
        thread::sleep(COOLDOWN);

        let task = tokio::spawn(async move {

            let content = process_request(&sido,&group).await;
            let file_name = FOLDER_GROUP.to_owned() + &group[..];
            save_to_file_json(&content,&file_name).await;     

            Ok(())
        });

        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await.expect("ERROR: Couldnt join task error");
    }

    Ok(())
}

async fn process_request(sido: &String, group: &String) -> Vec<Krotka> {
    let plain_html = get_plan_site(&sido, &group)
        .await
        .expect("ERROR: Couldnt get plan site");

    let krotkas = extract_krotkas(plain_html).await;

    krotkas
}

async fn save_to_file_json(content:&Vec<Krotka>,file_name:&str) {
    println!("Saved to file: {}",file_name);
    let mut file = File::create(file_name).await.expect("ERROR: Couldnt create file.");
    let content = serde_json::to_string(content).unwrap();
    file.write_all(content.as_bytes()).await.expect("ERROR: Couldnt save to file.");
}

//w 4 kolumnie text znajduje się sala
async fn extract_krotkas(html: String) -> Vec<Krotka> {
    let mut result: Vec<Krotka> = Vec::new();
    let selector = Selector::parse(r#"td[class="tdFormList1DSheTeaGrpHTM3"]"#).unwrap();
    let html = Html::parse_fragment(&html[..]);

    for td in html.select(&selector) {
        let td_title = td.value().attr("title").unwrap_or("").to_owned();
        let text = td.text().collect::<Vec<_>>();
        let class =text.get(4).unwrap_or(&"").to_owned().to_owned();

        let krotka = Krotka::new(td_title,class);
        result.push(krotka);
    }
    trasnsponse(result)
}

async fn extract_groups(sid:&String) -> Vec<String> {
    let html = get_plan_site(sid, "").await.expect("ERROR: Couldnt get groups site.");

    let mut result: Vec<String> = Vec::new();
    let selector = Selector::parse(r#"a[class=aMenu]"#).unwrap();
    let html = Html::parse_fragment(&html[..]);

    for a in html.select(&selector) {
        let group = a.text().next().unwrap();
        result.push(group.to_owned());
    }
    result
}

fn trasnsponse<T:Clone>(matrix: Vec<T>) -> Vec<T> {
    if matrix.len() == 0 {
        return matrix;
    }
    let mut new_matrix: Vec<T> = Vec::new();
    let mut i: usize = 0;
    let mut offset: usize = 0;

    loop {
        new_matrix.push(matrix[i].clone());
        i += VMAX;
        if i >= VMAX * HMAX {
            offset += 1;
            i = offset;
        }
        if new_matrix.len() == VMAX * HMAX {
            break;
        }
    }
    new_matrix
}

async fn get_sid(client: &reqwest::Client, url: &str) -> Result<String, reqwest::Error> {
    let body = client.get(url).send().await?.text().await?;
    let selector = Selector::parse(r#"form[name="aaa"]"#).unwrap();
    let html = Html::parse_fragment(&body[..]);

    let result = html
        .select(&selector)
        .next()
        .unwrap()
        .value()
        .attr("action")
        .expect("ERROR: Couldnt parse sid");

    if result == "" {
        panic!("ERROR: sid has expired");
    }

    let result: Vec<&str> = result.split('=').collect();
    let sid = String::from(result[1]);

    Ok(sid)
}

fn get_headers() -> reqwest::header::HeaderMap {
    let mut headers2 = reqwest::header::HeaderMap::new();
    let headers = [
        (
            "User-Agent",
            "Mozilla/5.0 (X11;Fedora; Linux x86_64; rv:73.0) Gecko/20100101 Firefox/73.0",
        ),
        ("Content-Type", "application/x-www-form-urlencoded"),
        ("Origin", "https://s1.wcy.wat.edu.pl"),
        ("Connection", "keep-alive"),
        ("Referer", "https://s1.wcy.wat.edu.pl/ed1/"),
        ("Upgrade-Insecure-Requests", "1"),
        ("Pragma", "no-cache"),
        ("Cache-Control", "no-cache"),
    ];

    for x in &headers {
        headers2.insert(x.0, x.1.parse().unwrap());
    }
    headers2
}

fn build_client() -> Result<reqwest::Client, reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    Ok(client)
}

async fn login(
    client: &reqwest::Client,
    sid: &str,
    user_id: String,
    password: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let form = &[
        ("formname", "login"),
        ("default_fun", "1"),
        ("userid", &user_id[..]),
        ("password", &password[..]),
        ("view_height", "1080"),
        ("view_width", "1920"),
    ];
    let headers = get_headers();

    let mut url: String = String::from("https://s1.wcy.wat.edu.pl/ed1/index.php?sid=");
    url.push_str(sid);

    let post = client
        .post(&url[..])
        .form(form)
        .headers(headers)
        .send()
        .await?;

    Ok(post)
}

async fn get_plan_site(sid: &String, group: &str) -> Result<String, reqwest::Error> {
    let client = build_client().unwrap();

    let mut url: String = String::from(
        "https://s1.wcy.wat.edu.pl/ed1/logged_inc.php?mid=328&iid=20192&pos=0&rdo=1&sid=",
    );
    let mut group_base: String = String::from("&exv=");
    url.push_str(&sid[..]);
    group_base.push_str(group);
    url.push_str(&group_base[..]);

    println!("{}", url);

    let post = client.post(&url[..]).send().await?.text().await?;
    Ok(post)
}
