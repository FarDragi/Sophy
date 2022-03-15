package database

import (
	"errors"

	"github.com/jackc/pgconn"
)

var (
	ViolateDuplicateKey = "23505"
)

func AsPgError(err error) (*pgconn.PgError, bool) {
	var pgerr *pgconn.PgError
	if ok := errors.As(err, &pgerr); ok {
		return pgerr, true
	}

	return nil, false
}

func VerifyError(pgerr *pgconn.PgError, violation string) bool {
	return pgerr.Code == violation
}
