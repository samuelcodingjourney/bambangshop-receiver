use rocket::serde::json::Json;

use bambangshop_receiver::Result;
use crate::model::notification::Notification;
use crate::model::subscriber::SubscriberRequest;
use crate::service::notification::NotificationService;



#[get("/subscribe/<product_type>")]
pub fn subscribe(product_Type: &str) -> Result<JSON<SubscriberRequest>> {
    return match NotificationService::subscribe(product_type) {
        Ok(f) => Ok(Json::from(f)),
        Err(e) => Err(e)
    };
}