use std::sync::Arc;
use async_trait::async_trait;
use crate::common::Mail;

#[async_trait]
pub trait Notifier<T> {
    fn OnMsg(&self, msg: Arc<Mail<T>>);
    fn OnConnect(&self);
    fn OnDisconnect(&self);
    fn OnError(&self, err: String);
}

pub type NotifierType<T> = Arc<dyn Notifier<T> + Sync + Send>;