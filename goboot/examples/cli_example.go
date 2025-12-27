// Example demonstrating CLI utilities in goboot.
package main

import (
	"fmt"
	"os"

	"dev.engineeringlabs/goboot/cli"
)

func main() {
	app := cli.NewApp("myapp", "A sample CLI application")
	cli.Version("1.0.0")(app)
	cli.Author("John Doe", "john@example.com")(app)

	// Note: This is a demonstration of the API structure.
	// Full implementation would include command registration.

	fmt.Printf("App: %s\n", app.Name)
	fmt.Printf("Description: %s\n", app.Description)
	fmt.Printf("Version: %s\n", app.Version)
	fmt.Printf("Author: %s <%s>\n", app.Author, app.Email)

	// Example of how commands would be defined:
	/*
	app.Command("greet").
		Description("Greet a user").
		Flag("name", cli.String, "Name to greet", cli.Required).
		Flag("loud", cli.Bool, "Use uppercase").
		Action(func(ctx *cli.Context) error {
			name := ctx.String("name")
			greeting := fmt.Sprintf("Hello, %s!", name)
			if ctx.Bool("loud") {
				greeting = strings.ToUpper(greeting)
			}
			fmt.Println(greeting)
			return nil
		})

	app.Command("version").
		Description("Show version information").
		Action(func(ctx *cli.Context) error {
			fmt.Printf("%s version %s\n", app.Name, app.Version)
			return nil
		})

	app.Run(os.Args)
	*/

	if len(os.Args) > 1 && os.Args[1] == "--help" {
		fmt.Println("\nUsage: myapp [command] [flags]")
		fmt.Println("\nCommands:")
		fmt.Println("  greet   Greet a user")
		fmt.Println("  version Show version information")
		fmt.Println("\nFlags:")
		fmt.Println("  --help  Show this help message")
	}
}
