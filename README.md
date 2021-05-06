# :milky_way: REST WMI - A Conventer for Windows WMI

rest-wmi is a simple conventer for [Windows WMI](https://docs.microsoft.com/en-us/windows/win32/wmisdk/about-wmi) written in [rust](https://www.rust-lang.org/).

[![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fyusufpapurcu%2Frest-wmi&count_bg=%233DC8A4&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=Repo+Popularity&edge_flat=false)](https://hits.seeyoufarm.com)

Using wmi is hard if language haven't libraries about wmi. Because creating, initializing and querying wmi is little difficult. If you want see steps you can look at [here](https://docs.microsoft.com/en-us/windows/win32/wmisdk/developing-a-wmi-provider). This is the existence point of rest-wmi.

rest-wmi designed for querying wmi above http server. Also can be used for monitoring remote clients. But in this conditions this may be dangerous.

## Using rest-wmi
Compile project with `cargo build` and double click `/target/debug/rest-wmi.exe`

You are ready to send json messages to `localhost:8080`

For an example send :

```json
{
    "namespace":"ROOT\\CIMV2",
    "query":"Select Name From Win32_Process"
}
```

Don't forget sending as `POST Request` !!

## Contributing to project
This project is a template and can be used for different cases. Because of this if you sent a case special thing this will not accept. But you can mail me and I add your project to users section :smile:

If you found a bug or useful thing can be added to template your contribution are welcome.

## Users 

If you use and want to be here please mail me from [here](mailto:yusufturhanp@gmail.com?Subject=I'm%20a%20rest-wmi%20user)
