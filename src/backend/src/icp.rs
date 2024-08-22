use candid::Principal;
use ic_cdk::{api, query, update};
use crate::global_state::{ORGANIZATIONS, PRODUCTS, USERS};
use crate::models::{Organization, Product, OrganizationInput, ProductInput, User, UserDetailsInput, UserResult};
use crate::utils::generate_unique_principal;
use crate::error::GenericError;


#[query]
pub fn get_organization_by_id(id: Principal) -> Option<Organization> {
    let organizations = ORGANIZATIONS.lock().unwrap();
    organizations.get(&id).cloned()
}

#[update]
pub fn create_organization(input: OrganizationInput) -> Organization {
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
pub fn update_organization(id: Principal, input: OrganizationInput) -> Option<Organization> {
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

impl Default for User {
    fn default() -> Self {
        User {
            id: api::caller(),
            org_ids: Vec::new(),
            is_principal: false,
            is_enabled: true,
            first_name: None,
            last_name: None,
            phone_no: None,
            email: None,
            address: None,
            detail_meta: Vec::new(),
            created_at: api::time(),
            created_by: api::caller(), // Default value for Principal
            updated_at: api::time(),
            updated_by: api::caller(), // Default value for Principal
        }
    }
}

#[update]
pub fn create_product(input: ProductInput) -> Product {
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
pub fn get_product_by_id(id: Principal) -> Option<Product> {
    let products = PRODUCTS.lock().unwrap();
    products.get(&id).cloned()
}

#[update]
pub fn update_product(id: Principal, input: ProductInput) -> Option<Product> {
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

#[update]
pub fn register() -> User {
    let mut users = USERS.lock().unwrap();
    let user = User {
        id: api::caller(),
        is_principal: users.is_empty(),
        ..Default::default()
    };
    users.insert(api::caller(), user.clone());
    user
}

#[query]
pub fn get_user_by_id(id: Principal) -> Option<User> {
    // TODO access control
    let users = USERS.lock().unwrap();
    users.get(&id).cloned()
}

#[query]
pub fn whoami() -> Option<User> {
    let users = USERS.lock().unwrap();
    users.get(&api::caller()).cloned()
}

#[update]
pub fn update_self_details(input: UserDetailsInput) -> UserResult {
    let mut users = USERS.lock().unwrap();
    if let Some(user) = users.get_mut(&api::caller()) {
        user.first_name = Some(input.first_name);
        user.last_name = Some(input.last_name);
        user.phone_no = Some(input.phone_no);
        user.email = Some(input.email);
        user.detail_meta = input.detail_meta;
        user.updated_at = api::time();
        user.updated_by = api::caller(); // Update with the current user
        UserResult::User(Some(user.clone()))
    } else {
        UserResult::Err(GenericError {
            message: "User not exist!".to_string(),
            ..GenericError::default()
        })
    }
}

#[update]
pub fn create_user(id: Principal, input: UserDetailsInput) -> UserResult {
    // TODO access control
    let mut users = USERS.lock().unwrap();

    if users.get(&id).is_some() {
        return UserResult::Err(GenericError {
            message: "User already exists!".to_string(),
            ..GenericError::default()
        })
    }

    let user = User {
        id: id,
        is_enabled: true,
        is_principal: false,
        first_name: Some(input.first_name),
        last_name: Some(input.last_name),
        email: Some(input.email),
        phone_no: Some(input.phone_no),
        detail_meta: input.detail_meta,
        ..Default::default()
    };
    ic_cdk::print(std::format!("User: {:?}", user.id.to_text()));
    
    users.insert(id, user.clone());
    UserResult::User(Some(user))
}

#[update]
pub fn update_user(id: Principal, input: UserDetailsInput) -> UserResult {
    // TODO access control
    let mut users = USERS.lock().unwrap();
    if let Some(user) = users.get_mut(&id) {
        user.first_name = Some(input.first_name);
        user.last_name = Some(input.last_name);
        user.phone_no = Some(input.phone_no);
        user.email = Some(input.email);
        user.detail_meta = input.detail_meta;
        user.updated_at = api::time();
        user.updated_by = api::caller(); // Update with the current user
        UserResult::User(Some(user.clone()))
    } else {
        UserResult::Err(GenericError {
            message: "User not found!".to_string(),
            ..GenericError::default()
        })
    }
}

#[update]
pub fn update_user_orgs(id: Principal, org_ids: Vec<Principal>) -> UserResult {
    // TODO access control
    let mut users = USERS.lock().unwrap();
    if let Some(user) = users.get_mut(&id) {
        user.org_ids = org_ids;
        UserResult::User(Some(user.clone()))
    } else {
        UserResult::Err(GenericError {
            message: "User not found!".to_string(),
            ..GenericError::default()
        })
    }
}


#[query]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}