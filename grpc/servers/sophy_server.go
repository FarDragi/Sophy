package servers

import (
	"context"

	"fardragi/sophy/grpc/database"
	"fardragi/sophy/grpc/pb"
	"fardragi/sophy/grpc/utils"

	"google.golang.org/grpc"
)

func RegisterSophyServer(server *grpc.Server) {
	pb.RegisterSophyServer(server, &BotServer{})
}

type BotServer struct {
	pb.UnimplementedSophyServer
}

func (s *BotServer) AddUserGlobalXp(ctx context.Context, req *pb.GlobalXpRequest) (*pb.GlobalXpResponse, error) {
	level, err := database.GlobalXpAddOrCreate(req.GetDiscordId(), 1)

	if err != nil {
		return &pb.GlobalXpResponse{}, err
	}

	if newLevel, newProgress, levelUp := utils.IsLevelUp(level.Level, level.Progress); levelUp {
		level, err := level.LevelUp(newLevel, newProgress)

		if err != nil {
			return &pb.GlobalXpResponse{}, err
		}

		return &pb.GlobalXpResponse{
			NewLevel: level.Level,
			LevelUp:  true,
		}, nil
	}

	return &pb.GlobalXpResponse{
		LevelUp: false,
	}, nil
}
