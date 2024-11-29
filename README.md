# Shuttle shared Postgres DB with Axum

This template shows how to connect a Postgres database and use it for a simple Book list app.

## Example usage

```bash
curl -X POST -H 'content-type: application/json' localhost:8000/books --data '{"title":"My book","isbn":"123-1111111111"}'
# {"id":1,"title":"My book","isbn":"123-1111111111"}

curl localhost:8000/books
# [{"id":1,"title":"My book","isbn":"123-1111111111"}]

curl localhost:8000/books/1
# {"id":1,"title":"My book","isbn":"123-1111111111"}
```
