package database

import (
	"time"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

var Connection *gorm.DB

func Connect() {
	db, err := gorm.Open(postgres.Open(""), &gorm.Config{})

	if err != nil {
		panic("Fail start database")
	}

	configDb, err := db.DB()

	if err != nil {
		panic("Fail get database config")
	}

	configDb.SetConnMaxIdleTime(10)
	configDb.SetMaxOpenConns(30)
	configDb.SetConnMaxIdleTime(time.Hour * 3)
	configDb.SetConnMaxLifetime(time.Hour * 10)

	Connection = db
}

func Migrate() {
	Connection.AutoMigrate()
}
