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
    pub fn list_all(product_type: &str) -> Vec<Subscriber> {
        if SUBSCRIBERS.get(product_type).is_none() {
            return vec![];
        };

        return SUBSCRIBERS
            .get(product_type)
            .unwrap()
            .iter()
            .map(|x| x.value().clone())
            .collect();
    }
    pub fn delete(product_type: &str, url: &str) -> Option<Subscriber> {
        if SUBSCRIBERS.get(product_type).is_none() {
            SUBSCRIBERS.insert(product_type.to_string(), Dashmap::new());
        };
        let result = SUBSCRIBERS.get(product_type).unwrap().remove(url);
        if !result.is_none() {
            return Some(result.unwrap().1);
        }
        return None;
    }
}
