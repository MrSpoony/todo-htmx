DATABASE_URL=postgresql://postgres:password@localhost:5435/todos

.PHONY: e2e
e2e:
	bunx --bun playwright test --trace on

.PHONY: reset-dev-db
reset-dev-db:
	sqlx database reset -f -y

.PHONY: start-dev-server
start-dev-server:
	cargo run

.PHONY: watch-dev-server
watch-dev-server:
	cargo watch -i e2e -x run

.PHONY: tailwindcss
tailwindcss:
	bunx --bun tailwindcss -i ./public/input.css -o ./public/output.css

.PHONY: watch-tailwindcss
watch-tailwindcss:
	bunx --bun tailwindcss -i ./public/input.css -o ./public/output.css --watch
