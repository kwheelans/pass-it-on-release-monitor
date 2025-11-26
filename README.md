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
uri = "sqlite://test_data/release-monitor.sqlite?mode=rwc"

[client]
key = "change me"

[[client.interface]]
type = "http"
host = "localhost"
port = 8080

[monitors]
[[monitors.monitor]]
type = "rancher-channel"
url = "https://update.rke2.io/v1-release/channels"
channel = "stable"
notification = "rke2"
frequency = 1
period = "day"

[[monitors.monitor]]
type = "rancher-channel"
url = "https://update.k3s.io/v1-release/channels"
channel = "stable"
notification = "k3s"

[[monitors.monitor]]
type = "github"
owner = "rancher"
repo = "rancher"
notification = "rancher-release"
```
