import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type BalanceResult = { 'Ok' : bigint } |
  { 'Err' : string };
export interface GenericError {
  'message' : string,
  'details' : Array<Metadata>,
}
export type GetMyProfileResponse = { 'Ok' : User } |
  { 'Err' : string };
export interface Metadata { 'key' : string, 'value' : string }
export interface Organization {
  'id' : Principal,
  'updated_at' : bigint,
  'updated_by' : Principal,
  'metadata' : Array<Metadata>,
  'name' : string,
  'description' : string,
  'created_at' : bigint,
  'created_by' : Principal,
}
export interface OrganizationInput {
  'metadata' : Array<Metadata>,
  'name' : string,
  'description' : string,
}
export interface Product {
  'id' : Principal,
  'updated_at' : bigint,
  'updated_by' : Principal,
  'metadata' : Array<Metadata>,
  'name' : string,
  'org_id' : Principal,
  'description' : string,
  'created_at' : bigint,
  'created_by' : Principal,
  'category' : string,
}
export interface ProductInput {
  'metadata' : Array<Metadata>,
  'name' : string,
  'org_id' : Principal,
  'description' : string,
  'category' : string,
}
export type Result = { 'Ok' : bigint } |
  { 'Err' : string };
export interface Tokens { 'e8s' : bigint }
export interface TransferArgs { 'amount' : bigint }
export interface User {
  'id' : Principal,
  'updated_at' : bigint,
  'updated_by' : Principal,
  'org_ids' : Array<Principal>,
  'is_principal' : boolean,
  'is_enabled' : boolean,
  'created_at' : bigint,
  'created_by' : Principal,
  'email' : [] | [string],
  'address' : string,
  'first_name' : [] | [string],
  'detail_meta' : Array<Metadata>,
  'last_name' : [] | [string],
  'phone_no' : [] | [string],
}
export interface UserDetailsInput {
  'email' : string,
  'first_name' : string,
  'detail_meta' : Array<Metadata>,
  'last_name' : string,
  'phone_no' : string,
}
export type UserResult = { 'user' : [] | [User] } |
  { 'error' : GenericError };
export interface _SERVICE {
  'canister_account' : ActorMethod<[], Uint8Array | number[]>,
  'create_organization' : ActorMethod<[OrganizationInput], Organization>,
  'create_product' : ActorMethod<[ProductInput], Product>,
  'create_user' : ActorMethod<[Principal, UserDetailsInput], UserResult>,
  'get_balance' : ActorMethod<[], BalanceResult>,
  'get_my_profile' : ActorMethod<[], GetMyProfileResponse>,
  'get_organization_by_id' : ActorMethod<[Principal], [] | [Organization]>,
  'get_product_by_id' : ActorMethod<[Principal], [] | [Product]>,
  'get_user_by_id' : ActorMethod<[Principal], [] | [User]>,
  'greet' : ActorMethod<[string], string>,
  'register' : ActorMethod<[], User>,
  'save_my_profile' : ActorMethod<[string], GetMyProfileResponse>,
  'transfer' : ActorMethod<[TransferArgs], Result>,
  'update_organization' : ActorMethod<
    [Principal, OrganizationInput],
    Organization
  >,
  'update_product' : ActorMethod<[Principal, ProductInput], Product>,
  'update_self_details' : ActorMethod<[UserDetailsInput], User>,
  'update_user' : ActorMethod<[Principal, UserDetailsInput], UserResult>,
  'update_user_orgs' : ActorMethod<[Principal, Array<Principal>], UserResult>,
  'whoami' : ActorMethod<[], [] | [User]>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
