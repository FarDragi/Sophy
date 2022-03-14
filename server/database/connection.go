package database

import (
	"fardragi/sophy/server/config"
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

	configDb.SetConnMaxIdleTime(10)
	configDb.SetMaxOpenConns(30)
	configDb.SetConnMaxIdleTime(time.Minute * 30)
	configDb.SetConnMaxLifetime(time.Hour * 3)

	Connection = db
}

func Migrate() {
	Connection.AutoMigrate(&User{}, &GlobalXp{}, &Guild{})
}
