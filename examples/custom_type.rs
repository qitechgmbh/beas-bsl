use beas_bsl::{ Client, ClientConfig, api::{ Ordering, QueryOptions } };

pub fn main()
{
    let config_data = std::fs::read_to_string("config.json").unwrap();

    let config = serde_json::from_str::<ClientConfig>(&config_data).unwrap();

    let client = Client::new(config).unwrap();
    
    let options = QueryOptions::new()
        .top(5)
        .skip(0)
        .order_by("DocEntry", Ordering::Descending);
    
    let workorders =
        client
        .single_request()
        .production()
        .workorder()
        .get(options.clone())
        .unwrap();
        
    println!("Workorder: {:?}", workorders);
}

#[derive(Debug, Clone)]
struct CustomWorkorder
{
    pub doc_entry: u32,
    
    pub closed: Option<i32>,
}