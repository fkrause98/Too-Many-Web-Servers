# Too-Many-Web-Servers
Too many web servers!
## Quick Start

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
