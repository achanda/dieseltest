Run this by

```
docker-compose down -v && docker-compose up --build
```

Then create a new user

```
curl -X POST http://localhost:3000/users \
    -H "Content-Type: application/json" \
    -d '{"name":"John Doe","email":"john.doe@example.com"}'
```
