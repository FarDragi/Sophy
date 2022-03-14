package services

import (
	"context"

	"fardragi/sophy/server/database"
	"fardragi/sophy/server/pb"
	"fardragi/sophy/server/utils"

	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func RegisterSophyServer(server *grpc.Server) {
	pb.RegisterSophyServer(server, &BotServer{})
}

type BotServer struct {
	pb.UnimplementedSophyServer
}

func (s *BotServer) AddUserGlobalXp(ctx context.Context, req *pb.GlobalXpRequest) (*pb.GlobalXpResponse, error) {
	globalXp := database.GlobalXp{
		DiscordId: req.GetDiscordId(),
	}
	globalXp, err := globalXp.AddOrCreate(1)

	if err != nil {
		return &pb.GlobalXpResponse{}, status.Error(codes.Internal, err.Error())
	}

	if newLevel, newProgress, levelUp := utils.IsLevelUp(globalXp.Level, globalXp.Progress); levelUp {
		level, err := globalXp.LevelUp(newLevel, newProgress)

		if err != nil {
			return &pb.GlobalXpResponse{}, status.Error(codes.Internal, err.Error())
		}

		return &pb.GlobalXpResponse{
			NewLevel: level.Level,
			LevelUp:  true,
		}, nil
	}

	return &pb.GlobalXpResponse{
		LevelUp: false,
	}, status.Error(codes.Aborted, "Cooldown")
}

func (s *BotServer) GetGuildConfig(ctx context.Context, req *pb.GuildConfigRequest) (*pb.GuildConfigResponse, error) {
	guild := database.Guild{
		DiscordId: req.GetDiscordId(),
	}
	guild, err := guild.GetConfig()

	if err != nil {
		return &pb.GuildConfigResponse{}, status.Error(codes.Internal, err.Error())
	}

	return &pb.GuildConfigResponse{
		LevelUpFormat: guild.LevelUpFormat,
	}, nil
}

func (s *BotServer) SetGuildLevelUpFormat(ctx context.Context, req *pb.SetGuildLevelUpFormatRequest) (*pb.SetGuildResponse, error) {
	return &pb.SetGuildResponse{}, status.Error(codes.Unimplemented, "")
}
