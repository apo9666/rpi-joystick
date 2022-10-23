## Pinout
![pinout](/doc/pinout.png)

## Cross compile (windows)
Install toolchain
https://gnutoolchains.com/raspberry64/

Edit ~/.cargo/config

```
[target.aarch64-unknown-linux-gnu]
linker = "C:\\SysGCC\\raspberry64\\bin\\aarch64-linux-gnu-gcc-10.exe"
```


```
rustup target add aarch64-unknown-linux-gnu
```



## Build
```shell
cargo build --target=aarch64-unknown-linux-gnu --release
```