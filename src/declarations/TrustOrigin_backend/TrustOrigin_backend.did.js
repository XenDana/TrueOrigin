export const idlFactory = ({ IDL }) => {
  const Metadata = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const OrganizationInput = IDL.Record({
    'metadata' : IDL.Vec(Metadata),
    'name' : IDL.Text,
    'description' : IDL.Text,
  });
  const Organization = IDL.Record({
    'id' : IDL.Principal,
    'updated_at' : IDL.Nat,
    'updated_by' : IDL.Principal,
    'metadata' : IDL.Vec(Metadata),
    'name' : IDL.Text,
    'description' : IDL.Text,
    'created_at' : IDL.Nat,
    'created_by' : IDL.Principal,
  });
  return IDL.Service({
    'createOrganization' : IDL.Func([OrganizationInput], [Organization], []),
    'getOrganizationById' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(Organization)],
        ['query'],
      ),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'updateOrganization' : IDL.Func(
        [IDL.Principal, OrganizationInput],
        [Organization],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
