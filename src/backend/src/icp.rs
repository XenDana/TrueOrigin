use candid::Principal;
use ic_cdk::{api, query, update};
use crate::global_state::{ORGANIZATIONS, PRODUCTS, USERS};
use crate::models::{Metadata, Organization, OrganizationInput, Product, ProductInput, User, UserDetailsInput, UserResult};
use crate::utils::generate_unique_principal;
use crate::error::GenericError;
use regex::Regex;

use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext, TransformFunc,
};

use serde::{Serialize, Deserialize};
use serde_json::{self, Value};


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

#[ic_cdk::update]
async fn generate_product_review(product_id: Principal) -> Option<Product> {
    let mut products = PRODUCTS.lock().unwrap();
    if let Some(product) = products.get_mut(&product_id) {
        let latest_product_review_generation = product.metadata.iter().find(|v| v.key == "latest_product_review_generation").map(|v| v.value.clone().parse::<u64>().ok()).flatten();
        if latest_product_review_generation.is_none() || latest_product_review_generation.unwrap() < api::time() - 86400 {
            // call scrape function
            // and update product data
            let product_reviews = scrape_product_review(product).await;

            let OPENAI_API_KEY = "OPEN_AI_API_KEY";
            let host = "api.openai.com";
            let url = format!(
                "https://{}/v1/chat/completions",
                host
            );

            let request_headers = vec![
                HttpHeader {
                    name: "Host".to_string(),
                    value: format!("{host}:443"),
                },
                HttpHeader {
                    name: "User-Agent".to_string(),
                    value: "exchange_rate_canister".to_string(),
                },
                HttpHeader {
                    name: "Content-Type".to_string(),
                    value: "application/json".to_string()
                },
                HttpHeader {
                    name: "Authorization".to_string(),
                    value: format!("Bearer {}", OPENAI_API_KEY)
                },
                HttpHeader {
                    name: "Idempotency-Key".to_string(),
                    value: generate_unique_principal(Principal::anonymous()).to_string()
                }
            ];

            let product_reviews_escaped = product_reviews.replace("\"", "\\\"");

            let json_data = format!(r#"
            {{
                "model": "gpt-4o",
                "messages": [
                    {{
                        "role": "user", "content": "With this product review summary: {}\n Please help summarize what is the overall sentiment of the product"
                    }}
                ],
                "temperature": 0.7
            }}
            "#, product_reviews_escaped);

            let json_utf8: Vec<u8> = json_data.as_bytes().to_vec(); // Convert JSON string to Vec<u8>
            let request_body: Option<Vec<u8>> = Some(json_utf8);

            //note "CanisterHttpRequestArgument" and "HttpMethod" are declared in line 4
            let request = CanisterHttpRequestArgument {
                url: url.to_string(),
                method: HttpMethod::POST,
                body: request_body,
                max_response_bytes: None,
                transform: Some(TransformContext {
                    // The "method" parameter needs to have the same name as the function name of your transform function
                    function: TransformFunc(candid::Func {
                        principal: ic_cdk::api::id(),
                        method: "transform".to_string(),
                    }),
                    // The "TransformContext" function does need a context parameter, it can be empty
                    context: vec![],
                }),
                headers: request_headers,
            };

            let cycles = 230_949_972_000;

            match http_request(request, cycles).await {
                Ok((response,)) => {
                    let response_body = String::from_utf8(response.body).unwrap_or_default(); // Convert Vec<u8> to String
                    let parsed: Value = serde_json::from_str(&response_body).unwrap();
                    let content = &parsed["choices"][0]["message"]["content"];
                    let metadata: Metadata = Metadata { key: "product_review".to_string(), value: content.to_string() };
                    product.metadata.push(metadata);
                    return Some(product.clone());
                }
                Err((r, m)) => {
                    let message =
                        format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

                    ic_cdk::print(message);

                    return None;
                }
            }
        }
    }

    ic_cdk::print(format!("Product not found"));

    return None;
}

async fn scrape_product_review(product: &Product) -> String {
    // let product_url = product.metadata.iter().find(|v| v.key == "ecommerce_url").map(|v| v.value.clone()).unwrap_or_default();

    // let json_data = format!(r#"
    // {{
    //     "url": "{}",
    //     "product_id: "{}"
    // }}
    // "#, product_url, product.id);

    // let json_utf8: Vec<u8> = json_data.as_bytes().to_vec(); // Convert JSON string to Vec<u8>
    // let request_body: Option<Vec<u8>> = Some(json_utf8);
    
    let request = CanisterHttpRequestArgument {
        url: format!("https://3a31-114-122-138-100.ngrok-free.app/product-review?id={}", product.id.to_string()),
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: Some(TransformContext {
            function: TransformFunc(candid::Func {
                principal: ic_cdk::api::id(),
                method: "transform".to_string(),
            }),
            context: vec![],
        }),
        headers: vec![],
    };

    let cycles = 230_949_972_000;

    match http_request(request, cycles).await {
        Ok((response,)) => {
            let response_body = String::from_utf8(response.body).unwrap_or_default(); // Convert Vec<u8> to String
            return response_body;
        }

        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            ic_cdk::print(message);

            return "No product review!".to_string();
        }
    }
}

#[query]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];

    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
    };

    if res.status == 200u64 {
        res.body = raw.response.body;
    } else {
        ic_cdk::api::print(format!("Received an error: err = {:?}", raw));
    }
    res
}