use reqwest::StatusCode;
use reverse_geocoder::ReverseGeocoder;

// This function uses the native reverse geocoding mechanism which should be
// wicked fast. There is another function below which assumes you have started 
// a local reverse geocoding server to resolve the queries
#[allow(dead_code)]
pub fn wicked_fast(lat: f64, lon: f64) -> String {
    let geocoder = ReverseGeocoder::new();
    let coords = (lat, lon);
    let r = geocoder.search(coords).record;
    format!("{}, {}, {}, {}", r.name, r.admin1, r.admin2, r.cc)
}

// This function assumes that you have started a local reverse geocoding server 
// running on port 3000 (the default). This is normally pretty darned quick
// and I can't really tell the difference in time between this function and the
// one above. Change the call in udp_broadcaster.rs near the bottom to try
// different calls and see which one is fastest for you.
#[allow(dead_code)]
#[tokio::main]
pub async fn from_http(lat: f64, lon: f64) -> String {
    let request_url = format!(
        "http://localhost:3000/?lat={:.4}&long={:.4}",
        lat, lon
    );
    #[allow(unused_assignments)]
    let mut ret_val = "".to_string();
    match reqwest::get(request_url).await {
    	Ok(response) => {
		    match response.status() {
		        StatusCode::OK => {
		            // Grab raw JSON from the http response
		            let parsed = json::parse(&response.text().await.unwrap()).unwrap();
		            ret_val = format!("{}, {}, {}, {}", parsed["name"], parsed["admin1"], parsed["admin2"], parsed["cc"]);
		        }
		        _ => {
		        	ret_val = "Unknown location.".to_string();
		        }
		    };
	    },
	    Err(_) => {
	    	ret_val = "Server Error.".to_string();
		}
    };
    ret_val
}
