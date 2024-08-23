use candid::Principal;
use ic_cdk::init;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use getrandom::register_custom_getrandom;
use std::time::Duration;
use std::{cell::RefCell, collections::HashMap};
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::models::{Organization, Product, User, Reseller, ProductSerialNumber, ProductVerification};

thread_local! {
    static RNG: RefCell<Option<StdRng>> = RefCell::new(None);
}

#[init]
fn init() {
    ic_cdk_timers::set_timer(Duration::ZERO, || ic_cdk::spawn(async {
        let (seed,): ([u8; 32],) = ic_cdk::call(Principal::management_canister(), "raw_rand", ()).await.unwrap();
        RNG.with(|rng| *rng.borrow_mut() = Some(StdRng::from_seed(seed)));
    }));
}

fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    RNG.with(|rng| rng.borrow_mut().as_mut().unwrap().fill_bytes(buf));
    Ok(())
}
register_custom_getrandom!(custom_getrandom);

lazy_static! {
    pub static ref ORGANIZATIONS: Mutex<HashMap<Principal, Organization>> = Mutex::new(HashMap::new());
    pub static ref PRODUCTS: Mutex<HashMap<Principal, Product>> = Mutex::new(HashMap::new());
    pub static ref USERS: Mutex<HashMap<Principal, User>> = Mutex::new(HashMap::new());
    pub static ref RESELLERS: Mutex<HashMap<Principal, Reseller>> = Mutex::new(HashMap::new());
    pub static ref PRODUCT_SERIAL_NUMBERS: Mutex<HashMap<Principal, Vec<ProductSerialNumber>>> = Mutex::new(HashMap::new());
    pub static ref PRODUCT_VERIFICATIONS: Mutex<HashMap<Principal, Vec<ProductVerification>>> = Mutex::new(HashMap::new());
}
