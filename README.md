# CompoundV2 Substreams

## Quick Start

### Install

Clone streamingfast/substreams repo.

Then, run `go install ./cmd/substreams` against the `develop` branch to get the latest `substreams` cli.

### Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Run

```bash
sftoken

# to run the map module
substreams run -e api-dev.streamingfast.io:443 substreams.yaml map_transfers --start-block 12292922 --stop-block +1

# to run the store module (and the map module in the background)
substreams run -e api-dev.streamingfast.io:443 substreams.yaml store_transfers --start-block 12292922 --stop-block +1
```

## Troubleshooting

Running `store_*` module could produce the below error. Just ignore it and retry.

```
Error: rpc error: code = Unknown desc = error building pipeline: synchronizing stores: from worker: calling back scheduler: squashing: merging partials: initializing next partial store "store_transfers": storage file 0012289000-0012288000.partial: not found
```
