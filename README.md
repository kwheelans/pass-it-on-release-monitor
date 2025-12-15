# pass-it-on-release-monitor

A pass-it-on client to send notifications when a configured monitor detects a new version has been released.

## Configuration

### Monitor Types

| Monitor         | Description                                                                                            |
|-----------------|--------------------------------------------------------------------------------------------------------|
| rancher-channel | Monitor the endpoint created my the [Rancher Channel Server](https://github.com/rancher/channelserver) |
| github          | Monitor Github repository releases                                                                     |


### Example
```toml
[global]
persist = true
db_path = "/path/to/db/release-monitor.sqlite"
github_personal_token = "sometoken"

[webui]
port = 8080
listen_address = "0.0.0.0"
pico_css_use_cdn = true
pico_css_local_path = "css/"
pico_css_color = "Indigo"

[client]
key = "change me"

[[client.interface]]
type = "http"
host = "localhost"
port = 8080

[monitors]
[[monitors.monitor]]
type = "rancher-channel"
name = "RKE2"
url = "https://update.rke2.io/v1-release/channels"
channel = "stable"
notification = "rke2"
frequency = 1
period = "day"

[[monitors.monitor]]
type = "rancher-channel"
name = "K3S"
url = "https://update.k3s.io/v1-release/channels"
channel = "stable"
notification = "k3s"

[[monitors.monitor]]
type = "github"
name = "Rancher"
owner = "rancher"
repo = "rancher"
notification = "rancher-release"
```
