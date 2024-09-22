# Too-Many-Web-Servers
Too many web servers! A lot of them! Written in Rust!
## Quick Start
1. Install nix (left as an exercise for the reader)
2. ```make dev-setup```.
3. Run one of the examples below.
Extra step: See how each server behaves under the stress test with: ```make stress```
You can also list the available targets with ```make help```
### Simple Server
First, run:
```shell
make simple_server.bin
```
Then in another terminal, you should able to do:
```shell
curl localhost:3000
# Hello world!
```
### Non Blocking Server
First, run:
```shell
make non_blocking_server.bin
```
Then in another terminal, you should able to do:
```shell
for elem in {1..10}; curl "localhost:3000"
# Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!Hello world!%
```

### Multiplexed Server
