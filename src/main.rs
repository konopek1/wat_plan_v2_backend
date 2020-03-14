#[macro_use]
extern crate rouille;
use std::time::Duration;
mod scrap_wat;

static INTERVAL: u64 = 8; // co ile godzin odswizane

#[allow(unreachable_code)]
fn main() {
    let port = std::env::var("PORT").expect("not port set");
    println!("{}",String::from("Now listening on localhost:") + &port);

    std::thread::spawn(move || loop {
        scrap_wat::fetch_parse_plan().unwrap();
        std::thread::sleep(Duration::from_secs(3600 * INTERVAL));
    });

    rouille::start_server(String::from("0.0.0.0:") + &port, move |request| {
        router!(request,
            (GET) (/) => {
                // If the request's URL is `/`, we jump here.
                // This block builds a `Response` object that redirects to the `/hello/world`.
                let group = request.get_param("group").unwrap();
                if group.starts_with("W"){
                    let plan_json = std::fs::read_to_string(group).unwrap();
                    return rouille::Response::json(&plan_json)
                }
                rouille::Response::empty_404()
            },

            // The code block is called if none of the other blocks matches the request.
            // We return an empty response with a 404 status code.
            _ => rouille::Response::empty_404()
        )
    });
}
