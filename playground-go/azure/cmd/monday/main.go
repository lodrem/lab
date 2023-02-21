package main

import (
	"github.com/spf13/cobra"
)

func main() {
	rootCmd := &cobra.Command{}
	rootCmd.AddCommand(ManifestCmd())

	if err := rootCmd.Execute(); err != nil {
		panic(err)
	}
}
