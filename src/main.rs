mod utils;
use utils::mqtt_utils::MqttClient;

use esp_idf_svc::nvs::{EspDefaultNvs, EspNvs, EspNvsPartition};
use uuid::{uuid, Uuid};

fn main() {
    // It is necessary to call this function once, otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    std::panic::set_hook(Box::new(|info| {
        log::error!("Panic occurred: {}", info);
    }));

    log::info!("Starting...");

    let nvs_partition = EspNvsPartition::take().unwrap();
    let mut default_nvs = EspDefaultNvs::new(nvs_partition, "storage", true).unwrap();

    // Manage Device UUID
    const DEVICE_UUID_NAMESPACE: &str = "blueguardian.co";
    let mut device_uuid: Uuid;
    let mut device_uuid_buf = [0u8; 36];
    let device_uuid_result = default_nvs.get_str("uuid", &mut device_uuid_buf);
    match device_uuid_result {
        Ok(value) => {
            log::debug!("Device UUID already exists");
            match Uuid::parse_str(&value) {
                Ok(uuid) => device_uuid = uuid,
                Err(e) => {
                    log::debug!("Device UUID is invalid: {:?}", e);
                    device_uuid =
                        Uuid::new_v5(&Uuid::NAMESPACE_DNS, (b"{}", DEVICE_UUID_NAMESPACE));
                }
            }
        }
        Err(e) => {
            log::debug!(
                "Device UUID not set, or another error occurred when retrieving it: {:?}",
                e
            );
            device_uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, (b"{}", DEVICE_UUID_NAMESPACE));
        }
    }
    log::info!("Device UUID: {}", device_uuid);

    // MQTT Client instantiation
    let mut mqtt_client = MqttClient::new(device_uuid);
    mqtt_client.run();
    // Subscribe to the to appropriate topics
    // mqtt_client.subscribe_to_topic("devices/command");

    // Logic to gather the appropriate information, and publish to the appropriate topics
    // mqtt_client.publish_to_topic("devices/status", {"battery": "0.8", "overall": "ok"}});
    // mqtt_client.publish_to_topic("devices/location", {"coordinate": ["-31","133"]});
}
