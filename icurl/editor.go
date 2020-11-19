package icurl

import (
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
)

func Launch() ([]byte, error) {
	fPath, err := ioutil.TempFile("", "data*.json")
	if err != nil {
		fmt.Fprint(os.Stdout, fmt.Sprintf("failed make edit file. %s\n", err.Error()))
		return nil, err
	}
	defer os.Remove(fPath.Name())

	editor := os.Getenv("EDITOR")
	err = openEditor(editor, fPath.Name())
	if err != nil {
		fmt.Fprint(os.Stdout, fmt.Sprintf("failed open text editor. %s\n", err.Error()))
		return nil, err
	}

	content, err := ioutil.ReadFile(fPath.Name())
	if err != nil {
		fmt.Fprint(os.Stdout, fmt.Sprintf("failed read content. %s\n", err.Error()))
		return nil, err
	}

	return content, nil
}

func openEditor(program string, args ...string) error {
	c := exec.Command(program, args...)
	c.Stdin = os.Stdin
	c.Stdout = os.Stdout
	c.Stderr = os.Stderr
	return c.Run()
}
