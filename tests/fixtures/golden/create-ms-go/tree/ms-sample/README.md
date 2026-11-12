# ms-sample

Microservicio

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /health | Health check |
| GET | /ready | Readiness probe |

## Getting Started

```bash
make run
```

## Docker

```bash
make docker
docker run -p 8080:8080 ms-sample:latest
```

## Author

Test Author

## License

MIT
