package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	//print text
	fmt.Println("This application will clear the recent files list for 1 or more MS Office applications on macOS. \n Script by KF: https://github.com/kenf1 \n")
	fmt.Println("Enter the number of the application's recent list you would like to clear? \n 1. Word \n 2. Excel \n 3. PowerPoint \n 4. All \n")

	//path + names
	plistPaths := []string{"/Library/Containers/com.microsoft.Word/Data/Library/Preferences/com.microsoft.Word.securebookmarks.plist", "/Library/Containers/com.microsoft.Excel/Data/Library/Preferences/com.microsoft.Excel.securebookmarks.plist", "/Library/Containers/com.microsoft.Powerpoint/Data/Library/Preferences/com.microsoft.Powerpoint.securebookmarks.plist"}
	// names := []string{"Word", "Excel", "PowerPoint", "All"}

	//convert entry to int
	entry, err := reader.ReadString('\n')
	if err != nil {
		log.Fatal(err)
	}

	entry1 := strings.Replace(entry, "\n", "", -1)
	userChoice, err := strconv.Atoi(entry1)
	if err != nil {
		log.Fatal(err)
	}

	//convert user choice into index
	varIndex := userChoice - 1

	//int 0<x<4
	if userChoice > 0 && userChoice < 4 {
		path := os.Getenv("HOME") + plistPaths[varIndex]
		err1 := os.Remove(path)
		if err1 != nil {
			fmt.Println(err1)
			return
		}
		fmt.Println("Successfully cleared recent files list.")
	} else if userChoice == 4 {
		for _, item := range plistPaths {
			file := os.Getenv("HOME") + item
			os.Remove(file)
		}
		fmt.Println("Successfully cleared recent files list.")
	} else {
		fmt.Println("Choices are from 1 to 4. Please try again.")
	}
}
