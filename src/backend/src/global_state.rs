use candid::Principal;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::models::{Organization, Product, User};


lazy_static! {
    pub static ref ORGANIZATIONS: Mutex<HashMap<Principal, Organization>> = Mutex::new(HashMap::new());
    pub static ref PRODUCTS: Mutex<HashMap<Principal, Product>> = Mutex::new(HashMap::new());
    pub static ref USERS: Mutex<HashMap<Principal, User>> = Mutex::new(HashMap::new());
}