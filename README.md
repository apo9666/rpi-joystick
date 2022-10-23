## Pinout
![pinout](/doc/pinout.png)

## Cross compile x64 (windows)
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

### Build
```shell
cargo build --target=aarch64-unknown-linux-gnu --release
```


## Cross compile x32 (windows)
Install toolchain
https://gnutoolchains.com/raspberry/

Edit ~/.cargo/config

```
[target.armv7-unknown-linux-gnueabihf]
linker = "C:\\SysGCC\\raspberry\\bin\\arm-linux-gnueabihf-gcc-10.exe"
```

```
rustup target add armv7-unknown-linux-gnueabihf
```


### Build
```shell
cargo build --target=armv7-unknown-linux-gnueabihf --release
```