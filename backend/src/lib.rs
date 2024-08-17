use candid::{CandidType, Nat, Principal, Deserialize};
use ic_cdk::{init, query, update};

#[derive(CandidType, Deserialize)]
struct Metadata {
    key: String,
    value: String,
}

#[derive(CandidType, Deserialize)]
struct Organization {
    id: Principal,
    name: String,
    description: String,
    metadata: Vec<Metadata>,
    created_at: Nat,
    created_by: Principal,
    updated_at: Nat,
    updated_by: Principal,
}

#[derive(CandidType, Deserialize)]
struct OrganizationInput {
    name: String,
    description: String,
    metadata: Vec<Metadata>,
}


#[query]
fn getOrganizationById(id: Principal) -> Option<Organization> {
    Err("Not Implemented")
}

#[update]
fn createOrganization(input: OrganizationInput) -> Organization {
    Err("Not Implemented")
}

#[update]
fn updateOrganization(id: Principal, input: OrganizationInput) -> Organization {
    Err("Not Implemented")
}
