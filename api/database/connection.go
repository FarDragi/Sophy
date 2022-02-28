package database

import (
	"fardragi/sophy/config"
	"fardragi/sophy/models"
	"time"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

var Connection *gorm.DB

func Connect(config config.Model) {
	db, err := gorm.Open(postgres.Open(config.DatabaseUrl), &gorm.Config{})

	if err != nil {
		panic("Fail start database")
	}

	configDb, err := db.DB()

	if err != nil {
		panic("Fail get database config")
	}

	configDb.SetMaxIdleConns(5)
	configDb.SetMaxOpenConns(50)
	configDb.SetConnMaxLifetime(time.Hour)
	configDb.SetConnMaxIdleTime(time.Minute * 10)

	db.AutoMigrate(&models.User{})

	Connection = db
}
