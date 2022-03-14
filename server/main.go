package main

import (
	"fmt"
	"net"

	"fardragi/sophy/server/config"
	"fardragi/sophy/server/database"
	"fardragi/sophy/server/services"

	_ "github.com/joho/godotenv/autoload"
	"google.golang.org/grpc"
)

func main() {
	config := config.Setup()

	database.Connect(config)
	database.Migrate()

	conn, err := net.Listen("tcp", fmt.Sprintf(":%s", config.Port))

	if err != nil {
		panic("Falha ao iniciar o tpc")
	}

	app := grpc.NewServer()

	services.RegisterSophyServer(app)

	app.Serve(conn)
}
