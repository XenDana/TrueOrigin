use candid::{CandidType, Principal, Deserialize};
use ic_cdk::{init, api, query, update};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

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
    created_at: Nat,
    created_by: Principal,
    updated_at: Nat,
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
            created_at: Nat::from(0 as u64), // Default value for Nat
            created_by: Principal::anonymous(), // Default value for Principal
            updated_at: Nat::from(0 as u64), // Default value for Nat
            updated_by: Principal::anonymous(), // Default value for Principal
        }
    }
}

#[update]
fn createProduct(input: ProductInput) -> Product {
    let id = Principal::anonymous(); // Generate a unique ID for the
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
