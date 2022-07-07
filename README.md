# Crate upnp-rs

A Rust crate providing basic Universal Plug and Play (UPnP) protocol implementations. 

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.38-green.svg)
[![crates.io](https://img.shields.io/crates/v/upnp-rs.svg)](https://crates.io/crates/upnp-rs)
[![docs.rs](https://docs.rs/upnp-rs/badge.svg)](https://docs.rs/upnp-rs)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-upnp.svg)](https://github.com/johnstonskj/rust-upnp/stargazers)

Implements the core protocols of the  _UPnP Device Architecture_ (UDA), specifically the discovery protocol for
control points to search for, and devices to notify of, device and service availability.

> _UPnP technology allows devices to connect seamlessly and to simplify network implementation in the home and 
> corporate environments_ — [Open Connectivity Foundation](https://openconnectivity.org/developer/specifications/upnp-resources/upnp/).

## Usage

Add the following to your `Cargo.toml`; currently this crate has no optional features.

```toml
upnp-rs = "0.2"
```

## API

The main client interface is the `discovery` module that provides `search` and `notify` capabilities. Over time 
the `description` module will be completed for the parsing and generation of device description messages. The 
following diagram shows the basic structure of the library with the two API modules relying on implementations of 
HTTPMU/HTTPU and SOAP respectively.

```
,--------, discover ,--------,     advertise     ,--------,
|        |--------->| disco. |<------------------|        |
| Client |          '--------'                   | Server |
|  API   | understand   :    ,--------, describe |  API   |
|        |------------------>| descr. |<---------|        |
'--------'              :    '--------'          '--------'
                        :         :
                        :         V
                        :    ,--------,
                        :    |  SOAP  |
                        :    '--------'
                        :         :
                        V         :
                    ,--------,    :
                    | HTTPMU |    :
                    '--------'    :
                        :         :
                        V         V              TCP/UDP
    ,---------------------------------------------------,
```

## Example

```rust
use upnp_rs::SpecVersion;
use upnp_rs::ssdp::search::*;

let mut options = Options::default_for(SpecVersion::V10);
options.search_target = SearchTarget::RootDevices;

match search_once(options) {
    Ok(responses) => {
        println!("search returned {} results.", responses.len());
        for (index, response) in responses.iter().enumerate() {
            println!("{}: {:#?}", index, response);
        }
    }
    Err(error) => {
        println!("search failed with error: {:#?}", error);
    }
}
```
  
## Command-Line

The command-line tool `upnp` can be used to perform basic operations using the SSDP API. Primarily these are used
for testing, but the search command can be used for general purpose discovery.

The general form of the command is _network-options command command-options_, as shown below.

```bash
USAGE
    upnp [-v|--verbose]+ [--interface=] [--ip-version=] COMMAND

COMMANDS
    search [-s|--search-target=] [-d|--domain=] [-w|--max-wait=]
```

* `interface` this is the name of a local network interface such as `en0`.
* `ip-version` denotes that the client should only use IP version 4 or 6.
* `search-target` denotes the scope of the search, valid values are `all`, `root`, `device:`_`id`_, 
  `device-type:`_`id`_, or `service-type:`_`id`_, 
* `domain` a domain to use for device/service types other than the default `schemas-upnp-org`.
* `max-wait` the wait time for replies, sent to devices to prevent message flooding.

## Changes

**Version 0.2.0**

* Rewritten error handling to have more discrete errors and better reporting.
* Moved from [log](https://crates.io/crates/log) and
  [env_logger](https://crates.io/crates/env_logger) to
  [tracing](https://crates.io/crates/tracing) and
  [tracing_subscriber](https://crates.io/crates/tracing_subscriber).
* Upgraded dependencies, especially [pnet](https://crates.io/crates/pnet)
  which had a reported vulnerability.
* Moved all *protocol* constants into the `syntax` module.
  
**Version 0.1.0**

* Ability to issue multicast non-caching search command, with parsed results.
* Ability to send multicast device notifications.
* Command-line tool `upnp` to issue test commands.

## TODO

1. Finish parsing search results.
2. Support listening for notifications.
3. Support fetching device details.
    4. Support for sending notifications.
