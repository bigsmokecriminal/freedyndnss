# Free DynDnss.net Client

just a simple client for [DynDnns.net](https://dyndnss.net) 

## Installation

Download the release file or build it yourself.

```
git clone https://github.com/bigsmokecriminal/freedyndnss.git
cd freedyndnss
cargo build --release
```
File will be in `target/release` folder   
Create a config file (see `example/example_config.toml`).


You can also add it to your cronjobs for automated DynDns updates.

## Usage

```
./freedyndnss <path_to_config>
```

## Where to get update url, domain, and pass

Update Url should not change, just take the one from `example/example_config.toml`   
To get your creds (domain and pass) go to your [control panel](https://dyndnss.net/intern.php). 
- domain will be your subname
- pass will be your router password
```
./freedyndnss <path_to_config>
```

