package main

import (
	"net"

	"fardragi/sophy/grpc/database"
	"fardragi/sophy/grpc/pb"
	"fardragi/sophy/grpc/services"

	"google.golang.org/grpc"
)

func main() {
	database.Connect()

	conn, err := net.Listen("tcp", ":8020")

	if err != nil {
		panic("Falha ao iniciar o tpc")
	}

	app := grpc.NewServer()

	pb.RegisterBotServer(app, &services.BotServer{})

	app.Serve(conn)
}
