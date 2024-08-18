use candid::{CandidType, Principal, Deserialize};
use ic_cdk::{api, query, update};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use ic_cdk::api::time;
use sha2::{Sha256, Digest};

#[derive(CandidType, Deserialize, Clone)]
struct Metadata {
    key: String,
    value: String,
}

#[derive(CandidType, Deserialize,  Clone)]
struct Organization {
    id: Principal,
    name: String,
    description: String,
    metadata: Vec<Metadata>,
    created_at: u64,
    created_by: Principal,
    updated_at: u64,
    updated_by: Principal,
}

#[derive(CandidType, Deserialize)]
struct OrganizationInput {
    name: String,
    description: String,
    metadata: Vec<Metadata>,
}

lazy_static! {
    static ref ORGANIZATIONS: Mutex<HashMap<Principal, Organization>> = Mutex::new(HashMap::new());
}

impl Default for Organization {
    fn default() -> Self {
        Organization {
            id: Principal::anonymous(), // Default value for Principal
            name: String::new(),
            description: String::new(),
            metadata: Vec::new(),
            created_at: api::time(),
            created_by: api::caller(), // Default value for Principal
            updated_at: api::time(),
            updated_by: api::caller(), // Default value for Principal
        }
    }
}

#[query]
fn get_organization_by_id(id: Principal) -> Option<Organization> {
    let organizations = ORGANIZATIONS.lock().unwrap();
    organizations.get(&id).cloned()
}

#[update]
fn create_organization(input: OrganizationInput) -> Organization {
    let id = Principal::anonymous(); // Generate a unique ID for the organization
    let organization = Organization {
        id,
        name: input.name,
        description: input.description,
        metadata: input.metadata,
        ..Default::default()
    };
    let mut organizations = ORGANIZATIONS.lock().unwrap();
    organizations.insert(id, organization.clone());
    organization
}

#[update]
fn update_organization(id: Principal, input: OrganizationInput) -> Option<Organization> {
    let mut organizations = ORGANIZATIONS.lock().unwrap();
    if let Some(org) = organizations.get_mut(&id) {
        org.name = input.name;
        org.description = input.description;
        org.metadata = input.metadata;
        org.updated_at = api::time();
        org.updated_by = api::caller(); // Update with the current user
        Some(org.clone())
    } else {
        None
    }
}

#[derive(CandidType, Deserialize,  Clone)]
struct Product {
    id: Principal,
    name: String,
    org_id: Principal,
    category: String,
    description: String,
    metadata: Vec<Metadata>,
    created_at: u64,
    created_by: Principal,
    updated_at: u64,
    updated_by: Principal,
}

#[derive(CandidType, Deserialize)]
struct ProductInput {
    name: String,
    org_id: Principal,
    category: String,
    description: String,
    metadata: Vec<Metadata>,
}

lazy_static! {
    static ref PRODUCTS: Mutex<HashMap<Principal, Product>> = Mutex::new(HashMap::new());
}

impl Default for Product {
    fn default() -> Self {
        Product {
            id: Principal::anonymous(),
            name: String::new(),
            org_id: Principal::anonymous(),
            description: String::new(),
            category: String::new(),
            metadata: Vec::new(),
            created_at: api::time(),
            created_by: api::caller(), // Default value for Principal
            updated_at: api::time(),
            updated_by: api::caller(), // Default value for Principal
        }
    }
}

#[update]
fn create_product(input: ProductInput) -> Product {
    let id = generate_unique_principal(Principal::anonymous()); // Generate a unique ID for the
    let product = Product {
        id,
        org_id: input.org_id,
        name: input.name,
        category: input.category,
        description: input.description,
        metadata: input.metadata,
        ..Default::default()
    };
    let mut products = PRODUCTS.lock().unwrap();
    products.insert(id, product.clone());
    product
}

#[query]
fn get_product_by_id(id: Principal) -> Option<Product> {
    let products = PRODUCTS.lock().unwrap();
    products.get(&id).cloned()
}

#[update]
fn update_product(id: Principal, input: ProductInput) -> Option<Product> {
    let mut products = PRODUCTS.lock().unwrap();
    if let Some(product) = products.get_mut(&id) {
        product.org_id = input.org_id;
        product.name = input.name;
        product.description = input.description;
        product.category = input.category;
        product.metadata = input.metadata;
        product.updated_at = api::time();
        product.updated_by = api::caller(); // Update with the current user
        Some(product.clone())
    } else {
        None
    }
}

fn generate_unique_principal(principal: Principal) -> Principal {
    // Combine the principal text and the current time
    let input = format!("{}-{}", principal.to_text(), time());

    // Hash the combined input using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    // Take the first 29 bytes of the hash and convert it into a Principal
    let principal_bytes: [u8; 29] = result[0..29].try_into().expect("slice with incorrect length");

    Principal::from_slice(&principal_bytes)
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}