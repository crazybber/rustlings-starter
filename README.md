# rust-starter

a rust starter,learning by example, based on rustlings.

## instructions

all examples go on windows 10 x64 in vscode

maybe you need to install following firstly.

~~~bash
rustup component add rls rust-analysis
~~~

## Start on Windows(by scripts)

First, set `ExecutionPolicy` to `RemoteSigned`:

~~~ps
Set-ExecutionPolicy RemoteSigned
~~~

Then, you can run:

```ps
Invoke-WebRequest https://git.io/rustlings-win | Select-Object -ExpandProperty Content | Out-File $env:TMP/install_rustlings.ps1; Unblock-File $env:TMP/install_rustlings.ps1; Invoke-Expression $env:TMP/install_rustlings.ps1
```

To install Rustlings. Same as on MacOS/Linux, you will have access to the `rustlings` command after it.

## Start on Windows or *unix(Manually)

Basically: Clone the repository, checkout to the latest tag, run `cargo install`.

```bash
git clone https://github.com/rust-lang/rustlings
cd rustlings
git checkout tags/2.2.1 # or whatever the latest version is (find out at https://github.com/rust-lang/rustlings/releases/latest)
cargo install --force --path .
```

If there are installation errors, ensure that your toolchain is up to date. For the latest, run:

```bash
rustup update
```

Then, same as above, run `rustlings` to get started.

## more information

(how to setup a rust development enviroment](https://zhuanlan.zhihu.com/p/92172591)