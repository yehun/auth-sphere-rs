use actix_http::HttpMessage;
use actix_web::dev::ServiceRequest;
use auth_sphere_db::table::user_session::UserDevice;

mod common;
pub mod request;
pub mod response;
pub mod logging;
pub(crate) mod header;


#[derive(Debug, Clone)]
pub struct Authorization(pub String);

impl Authorization {
    pub fn get(req: &ServiceRequest) -> Option<Self> {
        req.extensions().get::<Authorization>().cloned()
    }
}

#[derive(Debug, Clone)]
pub enum DeviceType {
    Web,
    Android,
    Ios,
    Desktop,
    Unknown
}

impl From<DeviceType> for UserDevice {
    fn from(value: DeviceType) -> Self {
        match value {
            DeviceType::Web => UserDevice::Web,
            DeviceType::Android => UserDevice::Android,
            DeviceType::Ios => UserDevice::Ios,
            DeviceType::Desktop => UserDevice::Desktop,
            DeviceType::Unknown => UserDevice::Web
        }
    }
}

impl From<&str> for DeviceType {
    fn from(value: &str) -> Self {
        // match value {
        //     "web" => DeviceType::Web,
        //     "android" => DeviceType::Android,
        //     "ios" => DeviceType::Ios,
        //     "desktop" => DeviceType::Desktop,
        //     &_ => DeviceType::Unknown
        // }
        match value {
            "1" => DeviceType::Web,
            "2" => DeviceType::Android,
            "3" => DeviceType::Ios,
            "4" => DeviceType::Desktop,
            &_ => DeviceType::Unknown
        }
    }
}
