import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

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
export interface _SERVICE {
  'create_organization' : ActorMethod<[OrganizationInput], Organization>,
  'create_product' : ActorMethod<[ProductInput], Product>,
  'get_organization_by_id' : ActorMethod<[Principal], [] | [Organization]>,
  'get_product_by_id' : ActorMethod<[Principal], [] | [Product]>,
  'greet' : ActorMethod<[string], string>,
  'update_organization' : ActorMethod<
    [Principal, OrganizationInput],
    Organization
  >,
  'update_product' : ActorMethod<[Principal, ProductInput], Product>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
