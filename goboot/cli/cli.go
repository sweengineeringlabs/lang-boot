// Package cli provides command-line interface building utilities.
//
// This module enables building type-safe CLI applications with:
//   - Argument parsing
//   - Subcommands
//   - Help generation
//   - Environment variable binding
//
// Example:
//
//	app := cli.NewApp("myapp", "My CLI application")
//	app.Command("greet", "Greet someone", func(ctx *cli.Context) error {
//	    name := ctx.String("name")
//	    fmt.Printf("Hello, %s!\n", name)
//	    return nil
//	}).
//	    Flag("name", "Name to greet", cli.Required).
//	    Flag("loud", "Use uppercase", cli.Bool)
//
//	app.Run(os.Args)
package cli

import "dev.engineeringlabs/goboot/cli/api"

// Re-export API types
type (
	// App represents a CLI application.
	App = api.App
	// Command represents a CLI command.
	Command = api.Command
	// Context provides access to parsed arguments.
	Context = api.Context
	// Flag represents a command-line flag.
	Flag = api.Flag
	// FlagType represents the type of a flag.
	FlagType = api.FlagType
)

// Flag types
const (
	String   FlagType = api.String
	Int      FlagType = api.Int
	Bool     FlagType = api.Bool
	Float    FlagType = api.Float
	Duration FlagType = api.Duration
	StringSlice FlagType = api.StringSlice
)

// Flag options
var (
	Required = api.Required
	Hidden   = api.Hidden
	EnvVar   = api.EnvVar
	Default  = api.Default
)

// NewApp creates a new CLI application.
func NewApp(name, description string) *App {
	return &App{
		Name:        name,
		Description: description,
		Commands:    make(map[string]*Command),
	}
}

// Version sets the application version.
func Version(v string) func(*App) {
	return func(a *App) {
		a.Version = v
	}
}

// Author sets the application author.
func Author(name, email string) func(*App) {
	return func(a *App) {
		a.Author = name
		a.Email = email
	}
}
