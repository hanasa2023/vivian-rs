use std::{
    collections::HashMap,
    sync::{Arc, Mutex, atomic::AtomicUsize},
};
use tokio::sync::mpsc;

pub struct AppState {
    pub clients: Mutex<HashMap<usize, mpsc::Sender<String>>>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        Arc::new(AppState {
            clients: Mutex::new(HashMap::new()),
        })
    }
}

pub static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);
