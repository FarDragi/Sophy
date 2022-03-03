package services

import (
	"context"
	"fardragi/sophy/grpc/pb"
)

type BotServer struct {
	pb.UnimplementedBotServer
}

func (s *BotServer) UserAddXp(ctx context.Context, req *pb.AddXp) (*pb.LevelUp, error) {

	return &pb.LevelUp{
		Level: 5,
	}, nil
}
