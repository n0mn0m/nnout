# Overview

Simple [SledDB](http://sled.rs/introduction.html) API service layer.
Put a k/v in at an endpoint and retrieve it.

## Example

```bash
curl -i -X GET -H "Content-Type: application/json" http://localhost:8080/api/foo
curl -i -X PUT -H "Content-Type: application/json" -d '{"bar":"baz"}' http://localhost:8080/api/foo
curl -i -X DELETE -H "Content-Type: application/json" http://localhost:8080/api/foo
```
