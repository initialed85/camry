package internal

import (
	"encoding/json"
	"fmt"
	"os"
	"reflect"
	"strings"
)

type X struct {
	Var any
}

func GetEnvironment[T any](key string, required bool, defaultValue *T) (T, error) {
	rawValue := strings.TrimSpace(os.Getenv(key))
	if rawValue == "" {
		if required {
			return *new(T), fmt.Errorf("env var %#+v empty or unset", key)
		}

		if defaultValue != nil {
			return *defaultValue, nil
		} else {
			return *new(T), fmt.Errorf("env var %#+v default value nil", key)
		}
	}

	x := struct {
		Var T
	}{}

	err := json.Unmarshal(
		[]byte(fmt.Sprintf(`{"Var": %#+v}`, rawValue)),
		&x,
	)
	if err != nil {
		err = json.Unmarshal(
			[]byte(fmt.Sprintf(`{"Var": %v}`, rawValue)),
			&x,
		)
		if err != nil {
			return *new(T), fmt.Errorf(
				"failed to parse %#+v as %v",
				rawValue,
				reflect.TypeOf(*new(T)).Name(),
			)
		}
	}

	return x.Var, nil
}

func Ptr[T any](x T) *T {
	return &x
}
