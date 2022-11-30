package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

var required_fields = [7]string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
var validPasswords int = 0

func contains(slice []string, item string) bool {
	set := make(map[string]struct{}, len(slice))
	for _, s := range slice {
		set[s] = struct{}{}
	}

	_, ok := set[item]
	return ok
}

func main() {
	file, err := ioutil.ReadFile("input.txt")
	if err != nil {
		fmt.Println("File reading error", err)
		return
	}

	splits := strings.Split(string(file), "\n\n")
	for _, item := range splits {
		line_fields := make([]string, 0)
		array := strings.Split(strings.Join(strings.Split(item, "\n"), " "), " ")

		for _, field := range array {
			key := strings.Split(field, ":")

			if key[0] != " " {
				line_fields = append(line_fields, string(key[0]))
			}
		}

		missing_fields := make([]string, 0)
		for _, field := range required_fields {
			if !contains(line_fields, field) {
				missing_fields = append(missing_fields, field)
			}
		}

		if len(missing_fields) == 0 {
			validPasswords += 1
		}

	}

	fmt.Println(validPasswords)
}
