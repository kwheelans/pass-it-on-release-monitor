# Unreleased

# v0.6.0
## Breaking Changes
- Remove rustls-tls-native-roots feature
- Remove download-pico-css from the CLI and use the container-utils docker image isntead

## Changes
- update pass-it-on to 0.17
- update request to 0.13
- add rustls crate and specifically select the aws_lc_rs provider

# v0.5.1
## Changes
- Web UI redirect to main page after posting from add, edit or delete
- update `octocrab` dependency  to version 0.49
- - update `zip` dependency  to version 7

# v0.5.0
## Features
- Add Pico CSS path and colour selection
- Display time as local time


# v0.4.0
## Features
- Persist data with SQLite database
- Serve Web UI to create, update or delete monitors

# v0.3.1
- update `octocrab` dependency  to version 0.44.y

# v0.3.0
## Features
- add option to persist versions to file between startups

## Changes
- update `thiserror` dependency  to version 2.x.y
- update `octocrab` dependency  to version 0.43.y
- update rust edition to 2024

# v0.2.0
## Changes
- update pass-it-on to 0.16.0
- change to tracing from log for logging

# v0.1.1
## Changes
 - update pass-it-on to v0.15.1

# v0.1.0
## Features
- Monitor [Rancher Channel Server](https://github.com/rancher/channelserver) endpoint
- Monitor Github releases
