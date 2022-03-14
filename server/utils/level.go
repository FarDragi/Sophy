package utils

func CalcLevel(level int32) int32 {
	progressMultiplier := ((level-1)/5 + 1) * 20
	levelMultiplier := ((level - 1) % 5) + 1
	base := int32(0)

	for i := 0; i < ((int(level)-1)/5)+1; i++ {
		base += 100 * int32(i)
	}

	return (progressMultiplier * levelMultiplier) + base
}

func IsLevelUp(level int32, progress int64) (newLevel int32, newProgress int64, levelUp bool) {
	progressTarget := CalcLevel(level)
	if progress >= int64(progressTarget) {
		newLevel = level + 1
		newProgress = progress - int64(progressTarget)
		levelUp = true
		return
	}

	return
}
