use super::Device;

pub trait DeviceControllers {
    fn list() -> impl Future<Output = String>;
}

impl DeviceControllers for Device {
    async fn list() -> String {
        "Hello, World!".to_string()
    }
}
