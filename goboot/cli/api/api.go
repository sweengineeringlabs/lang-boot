// Package api defines the CLI module's public interfaces.
package api

// App represents a CLI application.
type App struct {
	Name        string
	Description string
	Version     string
	Author      string
	Email       string
	Commands    map[string]*Command
}

// Command represents a CLI command.
type Command struct {
	Name        string
	Description string
	Flags       []Flag
	Args        []Arg
	Action      func(*Context) error
	Subcommands map[string]*Command
}

// Flag represents a command-line flag.
type Flag struct {
	Name        string
	Short       string
	Description string
	Type        FlagType
	Default     interface{}
	Required    bool
	Hidden      bool
	EnvVar      string
}

// Arg represents a positional argument.
type Arg struct {
	Name        string
	Description string
	Required    bool
}

// FlagType represents the type of a flag.
type FlagType int

const (
	String FlagType = iota
	Int
	Bool
	Float
	Duration
	StringSlice
)

// Context provides access to parsed arguments.
type Context struct {
	App     *App
	Command *Command
	Args    []string
	Flags   map[string]interface{}
}

// String returns a string flag value.
func (c *Context) String(name string) string {
	if v, ok := c.Flags[name].(string); ok {
		return v
	}
	return ""
}

// Int returns an int flag value.
func (c *Context) Int(name string) int {
	if v, ok := c.Flags[name].(int); ok {
		return v
	}
	return 0
}

// Bool returns a bool flag value.
func (c *Context) Bool(name string) bool {
	if v, ok := c.Flags[name].(bool); ok {
		return v
	}
	return false
}

// Flag options
type FlagOption func(*Flag)

func Required(f *Flag) {
	f.Required = true
}

func Hidden(f *Flag) {
	f.Hidden = true
}

func EnvVar(name string) FlagOption {
	return func(f *Flag) {
		f.EnvVar = name
	}
}

func Default(value interface{}) FlagOption {
	return func(f *Flag) {
		f.Default = value
	}
}
