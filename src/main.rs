#[macro_use]
extern crate rouille;
use std::time::Duration;
mod scrap_wat;

static INTERVAL: u64 = 8; // co ile godzin odswizane

#[allow(unreachable_code)]
#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or("8888".to_owned());
    let user_id = std::env::var("USER").expect("UserId global var not set");
    let password = std::env::var("PASSWORD").expect("Password global var not set");

    let fetcher = scrap_wat::Fetcher::new(user_id,password).await;
    rouille::start_server(String::from("0.0.0.0:") + &port, move |request| {
        router!(request,
            (GET) (/) => {
                let group = request.get_param("group").unwrap();
                if !group.starts_with(".") && !group.starts_with("/"){
                    //JA PIER DO LE DX tyle czasu
                    //json_res =  fetcher.get_group().await; JAK NIE JAK TAK
                    return rouille::Response::json(&json_res).with_additional_header("Access-Control-Allow-Origin","*")
                }
                rouille::Response::empty_404().with_additional_header("Access-Control-Allow-Origin","*")
            },

            _ => {
                let response = rouille::match_assets(&request, "dist");

                if response.is_success() {
                    return response;
                }
                rouille::Response::empty_404().with_additional_header("Access-Control-Allow-Origin","*")
            }
        )
    });
}
