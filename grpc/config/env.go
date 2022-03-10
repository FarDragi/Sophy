package config

import "os"

type Model struct {
	DatabaseUrl string
	Port        string
}

func Setup() Model {
	return Model{
		DatabaseUrl: os.Getenv("APP_DATABASE_URL"),
		Port:        os.Getenv("APP_GRPC_PORT"),
	}
}
