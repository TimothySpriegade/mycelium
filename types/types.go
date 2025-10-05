package types

type validType int

const (
	IntegerType validType = iota
	StringType
)

var typesByName = map[string]validType{
	"int":    IntegerType,
	"string": StringType,
}

func isValidType(name string) bool {
	_, ok := typesByName[name]
	return ok
}
