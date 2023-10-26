# CORS Proxy Rust

Simple proxy to bypass CORS issues. This is built as a dev-only solution to enable prototyping against existing APIs without having to worry about CORS.

This module is created to solve this problem:

```
No 'Access-Control-Allow-Origin' header is present on the requested resource. Origin 'http://localhost:3000' is therefore not allowed access. If an opaque response serves your needs, set the request's mode to 'no-cors' to fetch the resource with CORS disable
```

## Getting Started

```
cargo install  --git https://github.com/marianoeramirez/cors-proxy-rust.git
```

**Simple Example**

API endpoint that we want to request that has CORS issues:

```
https://www.example.com/users
```

Start Proxy:

```
cors-proxy --proxy-url https://www.example.com
```

Then in your client use the API endpoint:

```
http://localhost:8080/users
```

End result will be a request to `https://www.example.com/users` without the CORS issues!


## Options

| Option         | Example               | Default |
| -------------- | --------------------- | ------: |
| --proxyUrl     | https://www.google.ie |         |
| --port         | 8010                  |    8080 |
