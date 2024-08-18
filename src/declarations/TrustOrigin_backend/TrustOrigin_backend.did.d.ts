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
export interface _SERVICE {
  'createOrganization' : ActorMethod<[OrganizationInput], Organization>,
  'getOrganizationById' : ActorMethod<[Principal], [] | [Organization]>,
  'greet' : ActorMethod<[string], string>,
  'updateOrganization' : ActorMethod<
    [Principal, OrganizationInput],
    Organization
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
