"""
PyBoot Examples - Parsing Utilities

Demonstrates JSON, YAML, and TOML parsing.
"""

from dev.engineeringlabs.pyboot.parsing import (
    parse_json,
    parse_yaml,
    parse_toml,
    ParseError,
    ParseResult,
    JsonParser,
    YamlParser,
    TomlParser,
)


# Example 1: JSON Parsing
print("=" * 50)
print("Example 1: JSON Parsing")
print("=" * 50)

json_content = '''
{
    "name": "myapp",
    "version": "1.0.0",
    "dependencies": {
        "requests": "^2.28.0",
        "pydantic": "^2.0.0"
    }
}
'''

data = parse_json(json_content)
print(f"Parsed JSON: {data}")
print(f"Name: {data['name']}")
print(f"Version: {data['version']}")
print()


# Example 2: Using Parser class with Result
print("=" * 50)
print("Example 2: Parser with Result Type")
print("=" * 50)

parser = JsonParser()

# Valid JSON
result = parser.parse('{"key": "value"}')
if result.is_ok:
    print(f"Valid JSON: {result.unwrap()}")

# Invalid JSON
result = parser.parse('{"key": invalid}')
if result.is_err:
    error = result.unwrap_err()
    print(f"Invalid JSON error: {error.message}")
    print(f"Line: {error.line}, Column: {error.column}")
print()


# Example 3: TOML Parsing (Python 3.11+)
print("=" * 50)
print("Example 3: TOML Parsing")
print("=" * 50)

toml_content = '''
[project]
name = "myapp"
version = "1.0.0"

[project.dependencies]
requests = "^2.28.0"

[tool.ruff]
line-length = 100
'''

try:
    data = parse_toml(toml_content)
    print(f"Parsed TOML: {data}")
    print(f"Project name: {data['project']['name']}")
except ParseError as e:
    print(f"TOML parsing error: {e}")
print()


# Example 4: YAML Parsing (requires pyyaml)
print("=" * 50)
print("Example 4: YAML Parsing")
print("=" * 50)

yaml_content = '''
server:
  host: localhost
  port: 8080
  
database:
  driver: postgresql
  host: db.example.com
  port: 5432
  
features:
  - authentication
  - caching
  - logging
'''

try:
    data = parse_yaml(yaml_content)
    print(f"Parsed YAML: {data}")
    print(f"Server host: {data['server']['host']}")
    print(f"Features: {data['features']}")
except ParseError as e:
    print(f"YAML parsing error: {e}")
print()


# Example 5: Error handling with unwrap_or
print("=" * 50)
print("Example 5: Safe Parsing with Defaults")
print("=" * 50)

parser = JsonParser()

# Returns default on error
config = parser.parse('invalid json').unwrap_or({"default": True})
print(f"With invalid input: {config}")

config = parser.parse('{"loaded": true}').unwrap_or({"default": True})
print(f"With valid input: {config}")
