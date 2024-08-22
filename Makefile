create-canisters:
	dfx canister create --all

deploy-provider:
	dfx deploy ic_siwe_provider --argument "( \
	    record { \
	        domain = \"127.0.0.1\"; \
	        uri = \"http://127.0.0.1:5173\"; \
	        salt = \"salt\"; \
	        chain_id = opt 1; \
	        scheme = opt \"http\"; \
	        statement = opt \"Login to the SIWE/IC demo app\"; \
	        sign_in_expires_in = opt 300000000000; /* 5 minutes */ \
	        session_expires_in = opt 604800000000000; /* 1 week */ \
	        targets = opt vec { \
	            \"$$(dfx canister id ic_siwe_provider)\"; \
	            \"$$(dfx canister id TrustOrigin_backend)\"; \
	        }; \
	    } \
	)"
deploy-ledger:
    dfx deploy icp_ledger_canister --argument "( \
        variant { \
            Init = record { \
                minting_account = \"$MINTER_ACCOUNT_ID\"; \
                initial_values = vec { \
                    record { \
                        \"$DEFAULT_ACCOUNT_ID\"; \
                        record { \
                            e8s = 10_000_000_000 : nat64; \
                        }; \
                    }; \
                }; \
                send_whitelist = vec {}; \
                transfer_fee = opt record { \
                    e8s = 10_000 : nat64; \
                }; \
                token_symbol = opt \"LICP\"; \
                token_name = opt \"Local ICP\"; \
            } \
        } \
    )"

upgrade-provider:
	dfx canister install ic_siwe_provider --mode upgrade --upgrade-unchanged --argument "( \
	    record { \
	        domain = \"127.0.0.1\"; \
	        uri = \"http://127.0.0.1:5173\"; \
	        salt = \"salt\"; \
	        chain_id = opt 1; \
	        scheme = opt \"http\"; \
	        statement = opt \"Login to the SIWE/IC demo app\"; \
	        sign_in_expires_in = opt 300000000000; /* 5 minutes */ \
	        session_expires_in = opt 604800000000000; /* 1 week */ \
	        targets = opt vec { \
	            \"$$(dfx canister id ic_siwe_provider)\"; \
	            \"$$(dfx canister id TrustOrigin_backend)\"; \
	        }; \
	    } \
	)"

deploy-backend:
	dfx deploy TrustOrigin_backend

deploy-frontend:
	pnpm install
	dfx deploy TrustOrigin_user_frontend

deploy-all: create-canisters deploy-provider deploy-ledger deploy-backend deploy-frontend

run-frontend:
	npm install
	npm run dev

clean:
	rm -rf .dfx
	rm -rf dist
	rm -rf node_modules
	rm -rf src/declarations
	rm -f .env
	cargo clean


