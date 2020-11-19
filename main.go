package main

import (
	"github.com/DuGlaser/icurl/icurl"
	"github.com/c-bata/go-prompt"
)

func main() {
	c, err := icurl.NewCompleter()
	if err != nil {
		panic(err)
	}

	p := prompt.New(
		icurl.Excutor,
		c.Complete,
		prompt.OptionPrefix(">>> curl "),
		prompt.OptionTitle("icurl"),
	)

	p.Run()
}
