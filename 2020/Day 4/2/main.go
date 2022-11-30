package main

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

var requiredFields = []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
var validPasswords int = 0
var eyeColors = []string{"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}

func contains(slice []string, item string) bool {
	set := make(map[string]struct{}, len(slice))
	for _, s := range slice {
		set[s] = struct{}{}
	}

	_, ok := set[item]
	return ok
}

func fieldValidation(input string) bool {
	r := false
	switch input[0:3] {
	case "byr":
		date, _ := strconv.Atoi(input[4:])
		if date >= 1920 && date <= 2002 {
			r = true
		}
	case "iyr":
		date, _ := strconv.Atoi(input[4:])
		if date >= 2010 && date <= 2020 {
			r = true
		}
	case "eyr":
		date, _ := strconv.Atoi(input[4:])
		if date >= 2020 && date <= 2030 {
			r = true
		}
	case "hgt":
		value := []string{input[4 : len(input)-2], input[len(input)-2:]}
		height, _ := strconv.Atoi(value[0])
		unit := value[1]
		switch unit {
		case "cm":
			if height >= 150 && height <= 193 {
				r = true
			}
		case "in":
			if height >= 59 && height <= 76 {
				r = true
			}
		}
	case "hcl":
		if strings.Contains(input[4:], "#") {
			a := regexp.MustCompile("^[a-f0-9]*$")
			if a.MatchString(input[5:]) {
				if len(input[5:]) == 6 {
					r = true
				}
			}
		}
	case "ecl":
		validColors := []string{"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}
		for _, color := range validColors {
			if strings.Contains(input[4:], color) {
				r = true
				break
			}
		}
	case "pid":
		a := regexp.MustCompile("^[0-9]*$")
		if a.MatchString(input[4:]) {
			if len(input[4:]) == 9 {
				r = true
			}
		}
	case "cid":
		r = true
	}
	return r
}

func main() {
	file, err := ioutil.ReadFile("input.txt")
	if err != nil {
		fmt.Println("File reading error", err)
		return
	}

	splits := strings.Split(string(file), "\n\n")
	for _, item := range splits {
		lineFields := make([]string, 0)
		array := strings.Split(strings.Join(strings.Split(item, "\n"), " "), " ")

		for _, field := range array {
			key := strings.Split(field, ":")

			if fieldValidation(field) {
				lineFields = append(lineFields, string(key[0]))
			}
		}

		missing_fields := make([]string, 0)
		for _, field := range requiredFields {
			if !contains(lineFields, field) {
				missing_fields = append(missing_fields, field)
			}
		}

		if len(missing_fields) == 0 {
			validPasswords += 1
		}

	}

	fmt.Println(validPasswords)
}
