use candid::{Principal};
use ic_cdk::{api, query, update};
use crate::global_state::{ORGANIZATIONS, PRODUCTS};
use crate::models::{Organization, Product, OrganizationInput, ProductInput};
use crate::utils::{generate_unique_principal};

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

#[query]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}