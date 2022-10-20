# sei-test-contract

This is the github repo for the sei backend test.

# Set up local Sei

Please follow the documentation here for setting up your local sei testing environment: https://docs.seinetwork.io/smart-contracts-and-local-development/set-up-a-local-network

You can also use this deployment script to automate set up Sei locally: https://github.com/sei-protocol/sei-chain/blob/master/scripts/initialize_local_test_node.sh

# Instantiating a contract

directory, so they assume seid is in your $PATH:

```
cargo build
```

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.6
```

# Deploy set test contract to Sei

```
cd scripts/deployment
chmod 755 sei_test_deploy.sh
./sei_test_deploy.sh
```

# Interact with Sei test contract

You can place order through the seid cli and query the result there:

```
# get owner

seid query wasm contract-state smart $contract_addr '{"get_owner": {}}'

# set owner

seid tx wasm execute $contract_addr '{"set_owner": {"owner": "new owner address"}}' --from "wallet name" --gas=4000000 --fees=1000000usei --chain-id sei-chain --broadcast-mode block

# deposit

seid tx wasm execute $contract_addr '{"deposit": {"receiver1": "receiver1 address", "receiver2": "receiver2 address"}}' --amount=10000usei --from "William_wallet" --gas=4000000 --fees=1000000usei --chain-id sei-chain --broadcast-mode block

# get withdrawable balance
seid query wasm contract-state smart $contract_addr '{"get_balance": {"account": "receiver1 address", "denom": "usei"}}'

# withdraw

If you deposit 10000usei then each receiver will receive 4900 usei.
Owner will receive 200usei because default fee is 2.

seid tx wasm execute $contract_addr '{"withdraw": {"coins": [{"denom": "usei", "amount": "4900" }]}}' --from "wallet name" --gas=4000000 --fees=1000000usei --chain-id sei-chain --broadcast-mode block

```
