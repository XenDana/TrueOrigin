export const idlFactory = ({ IDL }) => {
  const Metadata = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const OrganizationInput = IDL.Record({
    'metadata' : IDL.Vec(Metadata),
    'name' : IDL.Text,
    'description' : IDL.Text,
  });
  const Organization = IDL.Record({
    'id' : IDL.Principal,
    'updated_at' : IDL.Nat64,
    'updated_by' : IDL.Principal,
    'metadata' : IDL.Vec(Metadata),
    'name' : IDL.Text,
    'description' : IDL.Text,
    'created_at' : IDL.Nat64,
    'created_by' : IDL.Principal,
  });
  const ProductInput = IDL.Record({
    'metadata' : IDL.Vec(Metadata),
    'name' : IDL.Text,
    'org_id' : IDL.Principal,
    'description' : IDL.Text,
    'category' : IDL.Text,
  });
  const Product = IDL.Record({
    'id' : IDL.Principal,
    'updated_at' : IDL.Nat64,
    'updated_by' : IDL.Principal,
    'metadata' : IDL.Vec(Metadata),
    'name' : IDL.Text,
    'org_id' : IDL.Principal,
    'description' : IDL.Text,
    'created_at' : IDL.Nat64,
    'created_by' : IDL.Principal,
    'category' : IDL.Text,
  });
  const UserDetailsInput = IDL.Record({
    'email' : IDL.Text,
    'first_name' : IDL.Text,
    'detail_meta' : IDL.Vec(Metadata),
    'last_name' : IDL.Text,
    'phone_no' : IDL.Text,
  });
  const User = IDL.Record({
    'id' : IDL.Principal,
    'updated_at' : IDL.Nat64,
    'updated_by' : IDL.Principal,
    'org_ids' : IDL.Vec(IDL.Principal),
    'is_principal' : IDL.Bool,
    'is_enabled' : IDL.Bool,
    'created_at' : IDL.Nat64,
    'created_by' : IDL.Principal,
    'email' : IDL.Opt(IDL.Text),
    'first_name' : IDL.Opt(IDL.Text),
    'detail_meta' : IDL.Vec(Metadata),
    'last_name' : IDL.Opt(IDL.Text),
    'phone_no' : IDL.Opt(IDL.Text),
  });
  return IDL.Service({
    'create_organization' : IDL.Func([OrganizationInput], [Organization], []),
    'create_product' : IDL.Func([ProductInput], [Product], []),
    'create_user' : IDL.Func([IDL.Principal, UserDetailsInput], [User], []),
    'get_organization_by_id' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(Organization)],
        ['query'],
      ),
    'get_product_by_id' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(Product)],
        ['query'],
      ),
    'get_user_by_id' : IDL.Func([IDL.Principal], [IDL.Opt(User)], ['query']),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'register' : IDL.Func([], [User], []),
    'update_organization' : IDL.Func(
        [IDL.Principal, OrganizationInput],
        [Organization],
        [],
      ),
    'update_product' : IDL.Func([IDL.Principal, ProductInput], [Product], []),
    'update_self_details' : IDL.Func([UserDetailsInput], [User], []),
    'update_user' : IDL.Func([IDL.Principal, UserDetailsInput], [User], []),
    'update_user_orgs' : IDL.Func(
        [IDL.Principal, IDL.Vec(IDL.Principal)],
        [User],
        [],
      ),
    'whoami' : IDL.Func([], [IDL.Opt(User)], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
