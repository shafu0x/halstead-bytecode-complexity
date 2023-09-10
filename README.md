## halstead-bytecode-complexity

Calculate the [halstead metrics](https://en.wikipedia.org/wiki/Halstead_complexity_measures) for EVM bytecode.

Inspired by [this](https://twitter.com/devtooligan/status/1698588856340406416)

#### how to run

```
cargo run ${PATH_TO_BYTECODE_TXT_FILE} 
```

##### flags

--rm-metadata : Remove the metada if the bytecode comes from solc
