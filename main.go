package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"strings"

	"github.com/DuGlaser/icurl/icurl"
	"github.com/c-bata/go-prompt"
)

func executor(t string) {
	arg := strings.Split(t, " ")

	for i, a := range arg {
		if a == "--editor" {
			content, _ := icurl.Launch()
			var buf bytes.Buffer
			json.Compac(&buf, content)

			arg[i] = fmt.Sprintf("\"%s\"", buf.String())
		}
	}

	fmt.Print("cmd: curl ")
	for _, a := range arg {
		fmt.Printf("%s ", a)
	}
	fmt.Println()
	fmt.Println()

	cmd := exec.Command("curl", arg...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Run()
	return
}

func main() {
	c, err := icurl.NewCompleter()
	if err != nil {
		panic(err)
	}

	p := prompt.New(
		executor,
		c.Complete,
		prompt.OptionPrefix(">>> curl "),
		prompt.OptionTitle("icurl"),
	)

	p.Run()
}
