use reqwest;
use scraper::{Html, Selector};
use std::prelude::v1::Vec;
use tokio::fs::File;
use tokio::prelude::*;

const COOLDOWN: std::time::Duration = std::time::Duration::from_secs(1);
const URL: &str = "https://s1.wcy.wat.edu.pl/ed1/";
const VMAX: usize = 22;
const HMAX: usize = 49;

#[derive(serde::Serialize)]
pub struct Krotka {
    title: String,
}

impl Krotka {
    fn new(title:String) -> Krotka {
        return Krotka {
            title: title,
        };
    }
}
type Task = tokio::task::JoinHandle<std::result::Result<(), std::io::Error>>;

#[tokio::main]
pub async fn fetch_parse_plan() -> Result<(), reqwest::Error> {
    let client: reqwest::Client = build_client().unwrap();

    let sid = get_sid(&client, URL).await?;
    println!("sid:{}", sid);

    let user_id = std::env::var("USER").expect("UserId global var not set");
    let password = std::env::var("PASSWORD").expect("Password global var not set");

    login(&client, &sid, user_id, password).await?;
    let plain_site = get_plan_site(&sid, "").await?.unwrap();
    let groups = extract_groups(plain_site);

    let mut tasks: std::vec::Vec<Task> = Vec::new(); // no tak czy nie xDD

    for group in groups {
        let sido = sid.clone();
        std::thread::sleep(COOLDOWN);
        let task = tokio::spawn(async move {
            let plain_html = get_plan_site(&sido, &group)
                .await
                .expect("ERROR GET PLAN SITE")
                .unwrap();

            let titles = extract_tds_titles(plain_html).await;
            let titles = trasnsponse(titles);

            let mut file = File::create("groups/".to_owned() + &group[..]).await?;
            let mut vec_json: Vec<Krotka> = Vec::new();
            
            for title in titles {
                let krotka = Krotka::new(title.to_owned());
                vec_json.push(krotka);
            }
            file.write_all(serde_json::to_string(&vec_json).unwrap().as_bytes())
                .await?;

            Ok(())
        });
        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await.expect("Join task error");
    }

    Ok(())
}

async fn extract_tds_titles(html: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let selector = Selector::parse(r#"td[class="tdFormList1DSheTeaGrpHTM3"]"#).unwrap();
    let html = Html::parse_fragment(&html[..]);

    for td in html.select(&selector) {
        let td_title = td.value().attr("title").unwrap_or(" ");
        result.push(td_title.to_owned());
    }
    result
}

fn extract_groups(html: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let selector = Selector::parse(r#"a[class=aMenu]"#).unwrap();
    let html = Html::parse_fragment(&html[..]);

    for a in html.select(&selector) {
        let group = a.text().next().unwrap();
        result.push(group.to_owned());
    }
    result
}

fn trasnsponse(matrix: Vec<String>) -> Vec<String> {
    if matrix.len() == 0 {
        return matrix;
    }
    let mut new_matrix: Vec<String> = Vec::new();
    let mut i: usize = 0;
    let mut offset: usize = 0;

    loop {
        new_matrix.push(matrix[i].to_owned());
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
        .unwrap();

    if result == "" {
        panic!("Brak sidu!!");
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

//https://s1.wcy.wat.edu.pl/ed1/logged_inc.php?mid=328&iid=20192&exv=WCY18KY2S1&pos=0&rdo=1&t=6801700&sid=
async fn get_plan_site(sid: &String, group: &str) -> Result<Option<String>, reqwest::Error> {
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
    Ok(Some(post))
}
