package modules

import (
	"time"

	"github.com/dgrijalva/jwt-go"
)

type UserClaims struct {
	Id string
}

func CreateJwt(userClaims UserClaims) (string, error) {
	claims := jwt.MapClaims{}
	claims["id"] = userClaims.Id
	claims["exp"] = time.Now().Add(time.Minute * 15).Unix()

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)

	jwt, err := token.SignedString([]byte("foda"))

	if err != nil {
		return "", err
	}

	return jwt, nil
}
