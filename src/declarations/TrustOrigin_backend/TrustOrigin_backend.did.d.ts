import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface GenericError {
  'message' : string,
  'details' : Array<Metadata>,
}
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
  'create_organization' : ActorMethod<[OrganizationInput], Organization>,
  'create_product' : ActorMethod<[ProductInput], Product>,
  'create_user' : ActorMethod<[Principal, UserDetailsInput], UserResult>,
  'get_organization_by_id' : ActorMethod<[Principal], [] | [Organization]>,
  'get_product_by_id' : ActorMethod<[Principal], [] | [Product]>,
  'get_user_by_id' : ActorMethod<[Principal], [] | [User]>,
  'greet' : ActorMethod<[string], string>,
  'register' : ActorMethod<[], User>,
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
