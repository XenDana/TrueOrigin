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
  return IDL.Service({
    'create_organization' : IDL.Func([OrganizationInput], [Organization], []),
    'create_product' : IDL.Func([ProductInput], [Product], []),
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
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'update_organization' : IDL.Func(
        [IDL.Principal, OrganizationInput],
        [Organization],
        [],
      ),
    'update_product' : IDL.Func([IDL.Principal, ProductInput], [Product], []),
  });
};
export const init = ({ IDL }) => { return []; };
