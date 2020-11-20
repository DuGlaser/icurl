package icurl

import (
	"strings"

	"github.com/c-bata/go-prompt"
)

func NewCompleter() (*Completer, error) {
	return &Completer{}, nil
}

type Completer struct{}

func (c *Completer) Complete(d prompt.Document) []prompt.Suggest {
	if d.TextBeforeCursor() == "" {
		return []prompt.Suggest{}
	}

	args := strings.Split(d.TextBeforeCursor(), " ")
	w := d.GetWordBeforeCursor()

	for i := range args {
		if args[i] == "|" {
			return []prompt.Suggest{}
		}
	}

	if strings.HasPrefix(w, "--") {
		return optionCompleter(d)
	}

	if strings.HasPrefix(w, "-") {
		return shortOptionCompleter(d)
	}

	if s, f := c.completeOptionArguments(d); f {
		return s
	}

	return []prompt.Suggest{}
}

func getPreviousOption(d prompt.Document) (option string, found bool) {
	args := strings.Split(d.TextBeforeCursor(), " ")
	l := len(args)
	if l >= 2 {
		option = args[l-2]
	}
	if strings.HasPrefix(option, "-") {
		return option, true
	}

	return "", false
}

func (c *Completer) completeOptionArguments(d prompt.Document) ([]prompt.Suggest, bool) {
	option, found := getPreviousOption(d)

	if !found {
		return []prompt.Suggest{}, false
	}

	switch option {
	case "-X", "--request":
		return httpMethodArguments(d), true

	case "-d", "--data":
		return []prompt.Suggest{{Text: "--editor", Description: "Open the editor"}}, true
	}

	return []prompt.Suggest{}, false
}
