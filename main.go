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

func completer(in prompt.Document) []prompt.Suggest {
	s := []prompt.Suggest{
		{Text: "-X GET", Description: "HTTP GET method"},
		{Text: "-X POST", Description: "HTTP POST method"},
		{Text: "-X DELETE", Description: "HTTP DELETE method"},
		{Text: "-X PATCH", Description: "HTTP PATCH method"},
		{Text: "-X PUT", Description: "HTTP PUT method"},
		{Text: "-d", Description: "HTTP POST data"},
		{Text: "-I", Description: "Show document info only"},
		{Text: "-v", Description: "Make the operation more talkative"},
		{Text: "-h", Description: "This help text"},
		{Text: "-H", Description: "Pass custom header(s) to server"},
		{Text: "\"Content-Type: application/json\""},
		{Text: "--editor", Description: "Open editor"},
	}
	return prompt.FilterFuzzy(s, in.GetWordBeforeCursor(), true)
}

func main() {
	p := prompt.New(
		executor,
		completer,
		prompt.OptionPrefix(">>> curl "),
		prompt.OptionTitle("icurl"),
	)

	p.Run()
}
