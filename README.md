# jito-rpc-test

To run:

```
$ git clone https://github.com/rpcpool/jito-rpc-test
$ cd jito-rpc-test
$ echo "RPC_URL=<rpc_url>" >.env
$ cargo run
```

Sample output:

```
Finished dev [unoptimized + debuginfo] target(s) in 0.36s
Running `target/debug/jito`
running version 1.16.19
running on node AAccsdfa2312111e1e1231231
running simulate_bundle
result Response { context: RpcResponseContext { slot: 234043679, api_version: Some(RpcApiVersion(Version { major: 1, minor: 16, patch: 19 })) }, value: RpcSimulateBundleResult { summary: Succeeded, transaction_results: [] } }
```
