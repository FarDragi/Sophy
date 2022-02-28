package models

type User struct {
	DiscordId string `gorm:"primaryKey;type:varchar(20)"`
	CreatedAt int64  `gorm:"autoCreateTime"`
	UpdatedAt int64  `gorm:"autoUpdateTime"`
}
