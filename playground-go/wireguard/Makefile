.PHONY: server
server:
	GOOS=linux GOARCH=amd64 go build -o ./bin/server ./cmd/server
	scp ./bin/server ccc@oasis:~/server

.PHONY: client
client:
	go run ./cmd/client
