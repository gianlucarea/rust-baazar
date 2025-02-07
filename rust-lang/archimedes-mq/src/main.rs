use std::{thread, time::Duration};

use paho_mqtt as mqtt;
use rand::Rng;
use serde::{Serialize, Deserialize};
use tokio::{fs::OpenOptions, io::AsyncWriteExt, task, time};

const BROKER: &str = "mqtt://localhost:1883";
const CLIEND_ID: &str = "archimedes-mq";
const TOPIC: &str = "sensor/data";

#[tokio::main]
async fn main() {
    
    let create_opts = mqtt::CreateOptionsBuilder::new()
    .server_uri(BROKER)
    .client_id(CLIEND_ID)
    .finalize();

    let client = mqtt::Client::new(create_opts).expect("Failed to create MQTT client");

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
    .keep_alive_interval(Duration::from_secs(20))
    .clean_session(true)
    .finalize();

    client.connect(conn_opts).expect("Failed to connect to MQTT broker");
    println!("Connected to MQTT broker!");


    let mqtt_client = client.clone();
    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            let sensor_data = generate_sensor_data();
            let payload = serde_json::to_string(&sensor_data).unwrap();
            let msg = mqtt::Message::new(TOPIC, payload.clone(), mqtt::QOS_1);
            if let Err(e) = mqtt_client.publish(msg){
                eprintln!("🧨 Failed to publish: {}", e);
            } else {
                println!("📡 Published: {}", payload);
            }
            let json = serde_json::to_string(&sensor_data).expect("Failed to serialize data");
            let json = json + ",\n";
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("sensor_log.txt").await.expect("cannot open a file");
        
            file.write(json.as_bytes()).await.expect("Write Failed");
        }
    });

    loop {
        thread::sleep(Duration::from_secs(10));
    }
}

fn generate_sensor_data() -> SensorData {
    let mut rng = rand::rng();
    SensorData{
        temperature: rng.random_range(20.0..30.0),
        pressure: rng.random_range(40.0..60.0),
        humidity: rng.random_range(980.0..1020.0),
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}

#[derive(Serialize,Deserialize,Debug)]
struct SensorData{
    temperature: f32,
    pressure: f32,
    humidity: f32,
    timestamp: String,
}
