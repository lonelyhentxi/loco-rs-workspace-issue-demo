# loco-rs-workspace-issue-demo

When using loco-rs and rust workspace v2, there are a few inconvenient points, such as the paths to read config files and to generate migrations can not change to fit the workspace structure.

```shell
cargo loco-app1 start
# cargo loco-app1 xxxx
cargo loco-app2 start
# cargo loco-app2 xxxx
```