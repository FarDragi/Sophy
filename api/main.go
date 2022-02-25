package main

import (
	"fardragi/sophy/config"
	"fardragi/sophy/database"

	"github.com/gin-gonic/gin"
	_ "github.com/joho/godotenv/autoload"
)

func main() {
	config := config.Setup()

	database.Connect(config)

	app := gin.Default()

	app.Run()
}
