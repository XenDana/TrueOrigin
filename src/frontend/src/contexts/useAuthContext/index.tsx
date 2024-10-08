import React, { createContext, useContext, useState, useCallback, useMemo, useEffect } from 'react';
import { AuthClient } from '@dfinity/auth-client';

import {
  AuthContextInterface,
  AuthContextProviderProps,
} from './interface';
import { TrustOrigin_backend } from '../../../../declarations/TrustOrigin_backend';
import type { OrganizationInput, ResellerInput, User, UserDetailsInput, UserRole } from '../../../../declarations/TrustOrigin_backend/TrustOrigin_backend.did';
import { handleUserResult } from '../../utils';
import { Principal } from '@dfinity/principal';

const AuthContext = createContext({} as AuthContextInterface);

// eslint-disable-next-line react-refresh/only-export-components
export const useAuthContext = () => useContext(AuthContext);

const MAX_TTL = BigInt(7 * 24 * 60 * 60 * 1000 * 1000 * 1000);

export const AuthContextProvider: React.FC<AuthContextProviderProps> = ({children}) => {
  const [isAuthenticated, setIsAuthenticated] = useState<boolean>();
  const [authClient, setAuthClient] = useState<AuthClient>();
  const [profile, setProfile] = useState<User | null>(null);
  const IDENTITY_PROVIDER = useMemo(() => 
    process.env.DFX_NETWORK === 'local'
    ? `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943`
    : `https://identity.ic0.app`
  , [])

  const login = () => {
    if (!authClient) return;
    authClient.login({
      identityProvider: IDENTITY_PROVIDER,
      onSuccess: async () => {
        setIsAuthenticated(true);
        const user = await TrustOrigin_backend.register();
        setProfile(user);
      },
      maxTimeToLive: MAX_TTL,
    });
  };

  const getProfile = async () => {
    if (!authClient) return;

    const principal = authClient.getIdentity().getPrincipal();
    if (principal.isAnonymous()) {
        return setProfile(null);
    }

    const responseData = await TrustOrigin_backend.whoami();
    if ('err' in responseData) {
        setProfile(null);
    } else if (responseData.length > 0) {
        setProfile(responseData[0] as User);
    }
  };

  const createProfile = useCallback(async (input: Partial<UserDetailsInput>) => {
    if (!authClient) return;

    const responseData = await TrustOrigin_backend.register();
    if (input.first_name) { // only fill in the user details, if the user chooses to do so
        const result = handleUserResult(await TrustOrigin_backend.update_self_details(input as UserDetailsInput))
        if (result) {
          setProfile(result);
        }
    } else {
        setProfile(responseData);
    }
  }, [authClient]);

  const updateProfile = useCallback(async (input: UserDetailsInput) => {
    if (!authClient || !input.first_name) return;

    const result = handleUserResult(await TrustOrigin_backend.update_self_details(input as UserDetailsInput))
    if (result) {
      setProfile(result);
    }
  }, [authClient]);

  const setSelfRole = useCallback(async (userRole: UserRole) => {
    const result = handleUserResult(await TrustOrigin_backend.set_self_role(userRole));
    if (result) {
      setProfile(result);
    }
  }, [])

  const logout = useCallback(() => {
    if (!authClient) return;
    
    authClient.logout();
    setIsAuthenticated(false);
  }, [authClient]);

  const signinAsBrandOwner = useCallback(async (input: OrganizationInput) => {
    const result = handleUserResult(await TrustOrigin_backend.register_as_organization(input));
    if (result) {
      setProfile(result);
    }
  }, []);

  const signinAsReseller = useCallback(async (input: ResellerInput) => {
    const result = handleUserResult(await TrustOrigin_backend.register_as_reseller({
      ...input,
      org_id: Principal.anonymous(),
    }));
    if (result) {
      setProfile(result);
    }
  }, []);

  useEffect(() => {
    AuthClient.create().then(async (client) => {
      const isAnonymous = await client
        .getIdentity()
        .getPrincipal()
        .isAnonymous();

      setAuthClient(client);
      setIsAuthenticated(!isAnonymous);
      if (!isAnonymous) {
        getProfile();
      }
    });
  }, []);

  useEffect(() => {
    if (authClient) {
      getProfile();
    }
  }, [authClient]);

  const contextValue = {
    profile,
    authClient,
    isAuthenticated,
    createProfile,
    updateProfile,
    setSelfRole,
    signinAsBrandOwner,
    signinAsReseller,
    login,
    logout,
  };

  return (
    <AuthContext.Provider value={contextValue}>{children}</AuthContext.Provider>
  );
};
