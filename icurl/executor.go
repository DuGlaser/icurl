package icurl

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"strings"
)

func Excutor(t string) {
	t = strings.TrimSpace(t)

	if t == "" {
		return
	}

	if t == "exit" {
		os.Exit(0)
		return
	}

	arg := strings.Split(t, " ")

	for i, a := range arg {
		if a == "--editor" {
			content, _ := launch()
			var buf bytes.Buffer
			json.Compac(&buf, content)

			arg[i] = fmt.Sprintf("'%s'", buf.String())
		}
	}

	fmt.Print("cmd: curl ")
	for _, a := range arg {
		fmt.Printf("%s ", a)
	}
	fmt.Println()
	fmt.Println()

	s := strings.Join(arg, " ")
	fmt.Println(s)

	cmd := exec.Command("/bin/sh", "-c", "curl "+s)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		fmt.Printf("Got error: %s\n", err.Error())
	}
	fmt.Println()
	fmt.Println()
	return
}
