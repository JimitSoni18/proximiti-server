```bash
# Start postgresql server podman image:
podman run --rm --name pg -p 5432:5432 \
	-e POSTGRES_PASSWORD=welcome \
	postgres

# In another terminal (tab) run psql:
podman exec -it -u postgres pg psql
```
