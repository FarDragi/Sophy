package database

func (g Guild) GetConfig() (Guild, error) {
	result := Connection.FirstOrCreate(&g)

	if result.Error != nil {
		return Guild{}, result.Error
	}

	return g, nil
}
