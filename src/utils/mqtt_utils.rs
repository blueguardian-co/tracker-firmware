use rumqttc::{Client, Connection, MqttOptions, QoS};
use std::thread;
use std::time::Duration;
use uuid::Uuid;

pub struct MqttClient {
    client: Client,
    connection: Connection,
    device_uuid: Uuid,
}

impl MqttClient {
    // Constructor to create a new MQTT client
    pub fn new(device_uuid: Uuid) -> Self {
        let mut mqtt_options = MqttOptions::new(device_uuid, "mqtt.eclipseprojects.io", 1883);
        mqtt_options.set_keep_alive(Duration::from_secs(10));

        let (client, connection) = Client::new(mqtt_options, 10);

        MqttClient {
            client,
            connection,
            device_uuid,
        }
    }

    pub fn run(&mut self) {
        log::debug!("Starting MQTT client...");

        // Spawn a thread to handle incoming MQTT events
        let mut connections = self.connection.iter().cloned();
        thread::spawn(move || {
            for message in connections {
                if let Ok(message) = message {
                    println!("Received = {:?}", message);
                }
            }
        });

        thread::sleep(Duration::from_secs(60));
    }

    pub fn publish_to_topic(&mut self, topic: &str, payload: &str) {
        let payload = format!("Current location of {}", self.device_uuid);
        self.client
            .publish(topic, QoS::AtMostOnce, false, payload)
            .unwrap();
    }

    pub fn subscribe_to_topic(&mut self, topic: &str) {
        self.client.subscribe(topic, QoS::AtMostOnce).unwrap();
    }
}
