package ui

import (
	"fmt"
	"log"

	"github.com/fatih/color"
)

func mustPrintln(i int, err error) {
	if err != nil {
		log.Fatalf("Failed to print: %s", err)
	}
}

func Title(format string, args ...any) {
	mustPrintln(color.New(color.FgCyan, color.Bold).Println(fmt.Sprintf(format, args...)))
}

func Section(format string, args ...any) {
	mustPrintln(color.New(color.FgHiBlue, color.Bold).Println(fmt.Sprintf(format, args...)))
}

func Tag(keyValues ...any) {
	for i := 0; i < len(keyValues); i += 2 {
		mustPrintln(color.New(color.FgYellow, color.Bold).Printf("%s=", keyValues[i]))
		mustPrintln(color.New(color.FgWhite).Printf("%s\n", keyValues[i+1]))
	}
}

func Info(format string, args ...any) {
	mustPrintln(color.New(color.FgHiWhite).Println(fmt.Sprintf(format, args...)))
}

func Error(err error, format string, args ...any) {
	mustPrintln(color.New(color.FgRed, color.Italic).Println("==========================================="))
	mustPrintln(color.New(color.FgHiRed, color.Bold).Println(fmt.Sprintf(format, args...)))
	mustPrintln(color.New(color.FgMagenta, color.Bold).Println(err))
	mustPrintln(color.New(color.FgRed, color.Italic).Println("==========================================="))
}

func Success(format string, args ...any) {
	mustPrintln(color.New(color.FgGreen, color.Bold).Println(fmt.Sprintf(format, args...)))
}
