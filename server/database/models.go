package database

type ModelTimes struct {
	CreatedAt int `gorm:"not null"`
	UpdatedAt int
}

type User struct {
	DiscordId string `gorm:"primaryKey;not null;type:char(18)"`
	Identity  string `gorm:"text"`
	ModelTimes
}

type GlobalXp struct {
	DiscordId string `gorm:"primaryKey;not null;type:char(18)"`
	Progress  int64  `gorm:"not null;default:0"`
	Level     int32  `gorm:"not null;default:1"`
	ModelTimes
}

type Guild struct {
	DiscordId     string `gorm:"primaryKey;not null;type:char(18)"`
	LevelUpFormat string `gorm:"not null;default:%user% level up to %level%!"`
	ModelTimes
}
