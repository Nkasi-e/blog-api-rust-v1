.PHONY: migrate server fresh entity serv serve-1

migrate:
	sea-orm-cli migrate up

server:
	cargo run

fresh:
	sea-orm-cli migrate fresh

entity:
	sea-orm-cli generate entity -o entity/src

serv-1:
	watchexec -- cargo run 

serv:
	cargo watch -x run

all: migrate server fresh entity serv serv-1
