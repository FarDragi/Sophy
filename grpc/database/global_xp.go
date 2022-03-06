package database

import "time"

func GlobalXpAddOrCreate(discordId string, count int64) (GlobalXp, error) {
	var globalXp GlobalXp
	result := Connection.Where(GlobalXp{
		DiscordId: discordId,
	}).FirstOrCreate(&globalXp)

	if result.Error != nil {
		return GlobalXp{}, result.Error
	}

	if (globalXp.UpdatedAt + 300) < int(time.Now().Unix()) {
		globalXp.Progress += 1
		result := Connection.Save(&globalXp)

		if result.Error != nil {
			return GlobalXp{}, result.Error
		}
	}

	return globalXp, nil
}

func (g GlobalXp) LevelUp(newLevel int32, newProgress int64) (GlobalXp, error) {
	g.Level = newLevel
	g.Progress = newProgress
	result := Connection.Save(&g)

	if result.Error != nil {
		return GlobalXp{}, result.Error
	}

	return g, nil
}
