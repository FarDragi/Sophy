package main

import (
	"net"

	"fardragi/sophy/grpc/config"
	"fardragi/sophy/grpc/database"
	"fardragi/sophy/grpc/servers"

	_ "github.com/joho/godotenv/autoload"
	"google.golang.org/grpc"
)

func main() {
	config := config.Setup()

	database.Connect(config)
	database.Migrate()

	conn, err := net.Listen("tcp", ":8020")

	if err != nil {
		panic("Falha ao iniciar o tpc")
	}

	app := grpc.NewServer()

	servers.RegisterXpServer(app)

	app.Serve(conn)
}
