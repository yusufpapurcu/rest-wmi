# :milky_way: REST WMI - A Conventer for Windows WMI

rest-wmi is a simple conventer for [Windows WMI](https://docs.microsoft.com/en-us/windows/win32/wmisdk/about-wmi) written in [rust](https://www.rust-lang.org/).

Using wmi is hard if language haven't libraries about wmi. Because creating, initializing and querying wmi is little difficult. If you want see steps you can look at [here](https://docs.microsoft.com/en-us/windows/win32/wmisdk/developing-a-wmi-provider). This is the existence point of rest-wmi.

rest-wmi designed for querying wmi above rest api. Also can be used for monitoring remote clients. But in this conditions this may be dangerous.

## Using rest-wmi
Compile project with `cargo build` and double click `/target/debug/rest-wmi.exe`

You are ready to send json messages to `localhost:8080`.
