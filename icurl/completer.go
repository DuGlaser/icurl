package icurl

import "github.com/c-bata/go-prompt"

func NewCompleter() (*Completer, error) {
	return &Completer{}, nil
}

type Completer struct{}

func (c *Completer) Complete(d prompt.Document) []prompt.Suggest {
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
	return prompt.FilterFuzzy(s, d.GetWordBeforeCursor(), true)
}
