use ic_cdk::update;
use serde_bytes::ByteBuf;

use crate::{
    declarations::ic_siwe_provider::{ic_siwe_provider, GetAddressResponse},
    models::{UserDetailsInput,Metadata, User,UserResult},
    icp::create_user
};

#[update]
async fn save_my_profile(name: String) -> Result<User, String> {
    // Get the address of the caller from the siwe provider canister, return error if it fails. A failure
    // here means that the caller is not authenticated using the siwe provider. This might happen if the
    // caller uses an anonymous principal or has authenticated using a different identity provider.
    let address = get_address().await?;
    let user_details = UserDetailsInput {
        first_name: name,
        last_name: String::default(),
        phone_no: String::default(),
        email: String::default(),
        detail_meta: Vec::<Metadata>::default(),
        address: Some(address)
    };

    // If user has an address and thus is authenticated, create a profile and save it.
    let user_result: crate::models::UserResult = create_user(
        ic_cdk::caller(),
        user_details
    );
    match user_result {
        UserResult::User(user) => Ok(user),
        UserResult::None => Err("User not found".to_string()),
        UserResult::Err(err) => Err(format!("Error: {:?}", err)),
    }

}

pub async fn get_address() -> Result<String, String> {
    let response = ic_siwe_provider
        .get_address(ByteBuf::from(ic_cdk::caller().as_slice()))
        .await;
    let address = match response {
        Ok((inner_result,)) => {
            // Handle the inner Result (GetAddressResponse)
            match inner_result {
                GetAddressResponse::Ok(address) => address, // Successfully got the address
                GetAddressResponse::Err(e) => return Err(e), // Handle error in GetAddressResponse
            }
        },
        Err(_) => return Err("Failed to get the caller address".to_string()), // Handle ic_cdk::call error
    };

    // Return the calling principal and address
    Ok(address)
}
