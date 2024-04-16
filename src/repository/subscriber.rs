use crate::model::subscriber::Subscriber;
use dashmap::DashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SUBSCRIBERS: Dashmap<String, DashMap<String, Subscriber>> = Dashmap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
    pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
        let subscriber_value = subscriber.clone();
        if SUBSCRIBERS.get(product_type).is_none() {
            SUBSCRIBERS.insert(product_type.to_string(), Dashmap::new());
        }
        SUBSCRIBERS
            .get(product_type)
            .unwrap()
            .insert(subscriber.url.clone(), subscriber_value);
        subscriber
    }
}
