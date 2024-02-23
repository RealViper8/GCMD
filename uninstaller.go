package main

import (
	"fmt"
	"os"
)

func main() {
	var input string

	fmt.Println("\x1b[0;32mUninstall GCMD \x1b[0;36m(\x1b[0;31my, \x1b[0;32mn\x1b[0;36m)\x1b[0m")
	fmt.Scanf("%s", &input)

	switch input {
	case "y":
		fmt.Println("Why did you leave us? :( \nUninstalling...")
		uninstall()
	case "n":
		fmt.Println("Exiting...")
		os.Exit(0)
	default:
		fmt.Println("Invalid Input. Exiting...")
		os.Exit(0)
	}
}

func uninstall() {
	fmt.Println("Uninstalling...")
	os.RemoveAll("./")
}
