package config

import "os"

type Model struct {
	DatabaseUrl string
}

func Setup() Model {
	return Model{
		DatabaseUrl: os.Getenv("APP_DATABASE_URL"),
	}
}
