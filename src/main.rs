#[macro_use]
extern crate rouille;
use std::time::Duration;
mod scrap_wat;

static INTERVAL: u64 = 8; // co ile godzin odswizane
                          // jesli projekt bedzie sie roziwal przebudowa totalna

#[allow(unreachable_code)]
fn main() {
    let port = std::env::var("PORT").unwrap_or("8888".to_owned());

    std::thread::spawn(move || loop {
        scrap_wat::fetch_parse_plan().unwrap();
        std::thread::sleep(Duration::from_secs(3600 * INTERVAL));
    });

    rouille::start_server(String::from("0.0.0.0:") + &port, move |request| {
        router!(request,
            (GET) (/) => {
                let group = request.get_param("group");
                match group {
                    Some(group) => {
                        if !group.starts_with(".") && !group.starts_with("/"){
                        let plan_json = std::fs::read_to_string("groups/".to_owned()+&group[..]).unwrap();
                        return rouille::Response::json(&plan_json).with_additional_header("Access-Control-Allow-Origin","*")
                    }
                },
                    None => return  rouille::Response::redirect_303("/index.html")
                }

                rouille::Response::empty_404().with_additional_header("Access-Control-Allow-Origin","*")
            },

            _ => {
                let response = rouille::match_assets(&request, "dist");

                if response.is_success() {
                    return response;
                }
                rouille::Response::redirect_303("/index.html")
            }
        )
    });
}
