# pass-it-on-release-monitor

A pass-it-on client to send notifications when a configured monitor detects a new version has been released.

## Flow
### Overall
1. Startup and get initial configuration from static files
2. Insert any monitor records from static configuration. On conflict with name update configuration.
3. Start Monitoring task
4. Start Web UI task
5. Start pass it on client.

### Monitoring
1. Each time the monitoring process starts it selects all models in the database
2. Use the db timestamp to compare
3. 

### Web UI
1. Get routes and listener and run
2. All records listed on the main page with button to add each monitor type
3. Selecting a record from the table gives the option edit or delete that record
4. When adding a record and that name exists display Error message

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
