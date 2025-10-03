package main

import (
	"fmt"
	"os"
	"os/user"

	"mycelium/repl"
)

func main() {
	user, err := user.Current()
	if err != nil {
		panic(err)
	}
	fmt.Printf("Hello %s! This is the mycelium programming langauge!\n", user.Username)
	fmt.Printf("Have fun trying some commands\n")
	repl.Start(os.Stdin, os.Stdout)
}
