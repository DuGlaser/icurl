package icurl

import "github.com/c-bata/go-prompt"

func HttpMethodArguments(d prompt.Document) []prompt.Suggest {
	s := []prompt.Suggest{
		{Text: "GET", Description: "HTTP GET method"},
		{Text: "POST", Description: "HTTP POST method"},
		{Text: "DELETE", Description: "HTTP DELETE method"},
		{Text: "PUT", Description: "HTTP PUT method"},
		{Text: "PATCH", Description: "HTTP PATCH method"},
	}

	return prompt.FilterFuzzy(s, d.GetWordBeforeCursor(), true)
}
