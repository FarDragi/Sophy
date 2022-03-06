package database

import "time"

func GlobalXpAddOrCreate(discordId string, count int64) (GlobalXp, error) {
	var globalXp GlobalXp
	xp := Connection.Where(GlobalXp{
		DiscordId: discordId,
	}).FirstOrCreate(&globalXp)

	if xp.Error != nil {
		return GlobalXp{}, xp.Error
	}

	if (globalXp.UpdatedAt + 300) < int(time.Now().Unix()) {
		globalXp.Progress += 1
		Connection.Save(&globalXp)
	}

	return globalXp, nil
}

func (g GlobalXp) LevelUp(newLevel int32, newProgress int64) (GlobalXp, error) {
	g.Level = newLevel
	g.Progress = newProgress
	result := Connection.Where(GlobalXp{
		DiscordId: g.DiscordId,
	}).Updates(&g)

	if result.Error != nil {
		return GlobalXp{}, result.Error
	}

	return g, nil
}
