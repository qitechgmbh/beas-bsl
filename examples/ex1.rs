use beas_bsl::{Client, ClientConfig, api::{Ordering, QueryOptions}};

pub fn main()
{
    let config_data = std::fs::read_to_string("config.json").unwrap();

    let config = serde_json::from_str::<ClientConfig>(&config_data).unwrap();

    let client = Client::new(config).unwrap();
    
    for i in 0..10
    {
        println!("sample: {}", i);
        test_range(&client, i * 2000, 2000);
    }
}

pub fn test_range(client: &Client, skip: u64, count: u64)
{
    let options = QueryOptions::new()
        .top(count)
        .skip(skip)
        .order_by("DocEntry", Ordering::Descending)
        ;
    
    let workorders = client
        .request()
        .production()
        .workorder()
        .get(options.clone());
    
    println!("workorders: {:?}", workorders.is_ok());
    
    let workorder_boms = client
        .request()
        .production()
        .workorder_bom()
        .get(options.clone());
    
    println!("workorder_boms: {:?}", workorder_boms.is_ok());
    
    let workorder_pos = client
        .request()
        .production()
        .workorder_pos()
        .get(options.clone());
    
    println!("workorder_pos: {:?}", workorder_pos.is_ok());
    
    let qc_order = client
        .request()
        .quality_control()
        .qcorder()
        .get(options.clone());
    
    println!("qc_order: {:?}", qc_order.is_ok());
    
    let qc_order_measurement = client
        .request()
        .quality_control()
        .qcorder_measurement()
        .get(options.clone());
    
    println!("qc_order_measurement: {:?}", qc_order_measurement.is_ok());
}