package servers

import (
	"context"

	"fardragi/sophy/grpc/database"
	"fardragi/sophy/grpc/pb/xp"
	"fardragi/sophy/grpc/utils"

	"google.golang.org/grpc"
)

func RegisterXpServer(server *grpc.Server) {
	xp.RegisterXpServer(server, &XpServer{})
}

type XpServer struct {
	xp.UnimplementedXpServer
}

func (s *XpServer) AddUserGlobalXp(ctx context.Context, req *xp.AddGlobalXp) (*xp.LevelUp, error) {
	level, err := database.GlobalXpAddOrCreate(req.GetDiscordId(), req.GetCount())

	if err != nil {
		return &xp.LevelUp{}, err
	}

	if newLevel, newProgress, levelUp := utils.IsLevelUp(level.Level, level.Progress); levelUp {
		level, err := level.LevelUp(newLevel, newProgress)

		if err != nil {
			return &xp.LevelUp{}, err
		}

		return &xp.LevelUp{
			NewLevel: level.Level,
			LevelUp:  true,
		}, nil
	}

	return &xp.LevelUp{
		LevelUp: false,
	}, nil
}
