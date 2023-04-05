## Run diploma Contract Node
```bash
./diploma_node --log info,runtime::contracts=debug 2>&1
```
## Build Contract
```bash
cargo contract build
```

## Deploy Contract
```bash
cargo contract instantiate --constructor new  --suri //Alice --salt $(date +%s)
```

## Polkadotjs URL
```
https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/contracts
```