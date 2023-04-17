# Spin component composition example

This example is adapted from the [wasm-tools compose example](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasm-compose/example) to execute with Spin.

```
❯ cargo-component --version
cargo-component 0.1.0 (a5ed5ea16 2023-03-27)
```

This example demonstrates executing a composed component with Spin using `wasm-tools compose`.

For immediate dopamine, just run `make example` and then:

```
curl localhost:3000 -d 'Hello, world!' -H "Content-Type: text/plain" -v --compressed
*   Trying 127.0.0.1:3000...
* Connected to localhost (127.0.0.1) port 3000 (#0)
> POST / HTTP/1.1
> Host: localhost:3000
> User-Agent: curl/7.87.0
> Accept: */*
> Accept-Encoding: deflate, gzip
> Content-Type: text/plain
> Content-Length: 13
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-type: text/plain
< content-encoding: gzip
< content-length: 58
< date: Mon, 17 Apr 2023 19:41:32 GMT
< 
* Connection #0 to host localhost left intact
The request body was: Hello, world!% 
```