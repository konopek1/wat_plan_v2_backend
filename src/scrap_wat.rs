use reqwest;
use scraper::{Html, Selector};
use std::prelude::v1::Vec;
use tokio::fs::File;
use tokio::prelude::*;
use std::path::Path;
use std::result::Result;
const COOLDOWN: std::time::Duration = std::time::Duration::from_secs(7);
const URL: &str = "https://s1.wcy.wat.edu.pl/ed1/";
const VMAX: usize = 22;
const HMAX: usize = 49;
const GROUP_FOLDER: &str = "./group";
const MAX_AGE: std::time::Duration = std::time::Duration::from_secs(60*30);

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
pub struct Fetcher {
    sid:String,
    login:String,
    pass:String
}
impl Fetcher {
    async fn new(log:String,ps:String)->Fetcher{
        let client_tmp: reqwest::Client = build_client().unwrap();
        let sid_tmp = match get_sid(&client_tmp,URL).await{
            Ok(s) => s,
            Err(e) => panic!("Blad w uzyskiwaniu sid\n")
        };
        return Fetcher{
            sid:sid_tmp,
            login:log,
            pass:ps
        };
    }
    async fn get_group(&mut self,group:String)->Option<String>{
        //TODO zrobic to co w pseudokodzie
        /*
        if(nie istnieje plik grupy || zmodyfikowano dawniej niż X godzin)
            pobierz ze strony
            if(fail)
                update sid
                pobierz ze strony
                if(fail)
                    zjebało sie na ament
        
        pobierz dane z pliku
        wyslij odpowiedz
        */
        let path = Path::new(GROUP_FOLDER).join(&group);
       //let mut file = std::fs::File::open(path.join(&group));
        if !path.is_file()  {
            let r = self.fetch_group(&group).await;
            if r.is_err(){
                return None;
            }
        }
        let age = std::fs::metadata(&path).unwrap().modified().unwrap().elapsed().unwrap();
        if  age > MAX_AGE {
            let r = self.fetch_group(&group).await;
            if r.is_err(){
                return None;
            }
        }
        let ret = std::fs::read_to_string(path).unwrap();
        return Some(ret);
    }
    fn cache(filename:String,grstr:String){

    }
    async fn fetch_group(&mut self,group:&String) -> Result<(),std::io::Error>{
        let path = Path::new(GROUP_FOLDER);
        let plain_html = match get_plan_site(&self.sid, &group).await{
            Ok(h) => h,
            Err(e) => {
                self.update_sid().await;
                let html = get_plan_site(&self.sid,&group).await?;
                html
            }
        };
        let titles = extract_tds_titles(plain_html).await;
        let titles = trasnsponse(titles);
        let mut file = File::create(path.join(group)).await.expect("Cannot write to file in GROUP_FOLDER");
        let mut vec_json: Vec<Krotka> = Vec::new();

        for title in titles {
            let krotka = Krotka::new(title.to_owned());
            vec_json.push(krotka);
        }
        file.write_all(serde_json::to_string(&vec_json).unwrap().as_bytes())
            .await?;

        Ok(())
    }
    async fn update_sid(&mut self)
    {
        let client_tmp: reqwest::Client = build_client().unwrap();
        self.sid = get_sid(&client_tmp,URL).await.unwrap();
        let r = login(&client_tmp,&self.sid,self.login.clone(),self.pass.clone()).await.expect("blad logowania");
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
    let plain_site = get_plan_site(&sid, "").await.unwrap();
    let groups = extract_groups(plain_site);

    let mut tasks: std::vec::Vec<Task> = Vec::new(); // no tak czy nie xDD

    for group in groups {
        let sido = sid.clone();
        std::thread::sleep(COOLDOWN);
        let task = tokio::spawn(async move {
            let plain_html = get_plan_site(&sido, &group)
                .await
                .expect("ERROR GET PLAN SITE");

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
async fn get_plan_site(sid: &String, group: &str) -> Result<String, std::io::Error> {
    let client = build_client().unwrap();

    let mut url: String = String::from(
        "https://s1.wcy.wat.edu.pl/ed1/logged_inc.php?mid=328&iid=20192&pos=0&rdo=1&sid=",
    );
    let mut group_base: String = String::from("&exv=");
    url.push_str(&sid[..]);
    group_base.push_str(group);
    url.push_str(&group_base[..]);

    println!("{}", url);
    let response = match client.post(&url[..]).send().await{
        Ok(k) => k,
        Err(e) => return Err(io::Error::new(std::io::ErrorKind::Other,"jebany rust godzina poszla na to"))
    };
    if response.url().as_str().len() < 30
    {
        //to znaczy ze sid sie wyczerpał i wylądowaliśmy na https://wcy.wat.edu.pl
        return Err(io::Error::new(std::io::ErrorKind::Other,"sid sie skonczyl"));
    }
    let post = response.text().await.expect("blad w odczycie htmla");
    Ok(post)
}
