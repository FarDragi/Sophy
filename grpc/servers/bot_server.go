package servers

import (
	"context"

	"fardragi/sophy/grpc/database"
	"fardragi/sophy/grpc/pb"
	"fardragi/sophy/grpc/utils"

	"google.golang.org/grpc"
)

func RegisterXpServer(server *grpc.Server) {
	pb.RegisterBotServer(server, &BotServer{})
}

type BotServer struct {
	pb.UnimplementedBotServer
}

func (s *BotServer) AddUserGlobalXp(ctx context.Context, req *pb.AddGlobalXp) (*pb.LevelUp, error) {
	level, err := database.GlobalXpAddOrCreate(req.GetDiscordId(), req.GetCount())

	if err != nil {
		return &pb.LevelUp{}, err
	}

	if newLevel, newProgress, levelUp := utils.IsLevelUp(level.Level, level.Progress); levelUp {
		level, err := level.LevelUp(newLevel, newProgress)

		if err != nil {
			return &pb.LevelUp{}, err
		}

		return &pb.LevelUp{
			NewLevel: level.Level,
			LevelUp:  true,
		}, nil
	}

	return &pb.LevelUp{
		LevelUp: false,
	}, nil
}
