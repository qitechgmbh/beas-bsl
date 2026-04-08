use beas_bsl::api::{Ordering, QueryOptions};
use beas_bsl::{Client, ClientConfig};

pub fn main() {
    let config = ClientConfig::from_file("config.json").unwrap();
    let client = Client::new(config).unwrap();

    let options = QueryOptions::new()
        .top(5)
        .skip(0)
        .order_by("DocEntry", Ordering::Descending);

    // Requests::Production::Workorder::Get()
    let workorders = client
        .request_single()
        .production()
        .workorder()
        .get(options)
        .unwrap();

    println!("Workorder: {:?}", workorders);
}
