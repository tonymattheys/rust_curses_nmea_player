use reqwest::StatusCode;

#[tokio::main]
pub async fn whereami(lat: &str, lon: &str) -> String {
    let apikey = "1cee7a64052347b3b86cc1627b441718";
    let request_url = format!(
        "https://api.geoapify.com/v1/geocode/reverse?lat={}&lon={}&apiKey={}",
        lat, lon, apikey
    );
    let mut ret_val: String = "".to_string();
    let response = reqwest::get(request_url).await.unwrap();
    match response.status() {
        StatusCode::OK => {
            // Grab raw JSON from the http response
            let parsed = json::parse(&response.text().await.unwrap()).unwrap();
            for (k, v) in parsed.entries() {
                match k {
                    "features" => {
                        ret_val = format!(
                            "{}, {}, {}",
                            v[0]["properties"]["county"],
                            v[0]["properties"]["state"],
                            v[0]["properties"]["city"]
                        );
                    }
                    _ => {
                    }
                }
            }
        }
        _ => {
        }
    };
    ret_val
}
