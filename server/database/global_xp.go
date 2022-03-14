package database

import "time"

func (g GlobalXp) AddOrCreate(count int64) (GlobalXp, error) {
	result := Connection.FirstOrCreate(&g)

	if result.Error != nil {
		return GlobalXp{}, result.Error
	}

	if (g.UpdatedAt + 300) < int(time.Now().Unix()) {
		g.Progress += 1
		result := Connection.Save(&g)

		if result.Error != nil {
			return GlobalXp{}, result.Error
		}
	}

	return g, nil
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
