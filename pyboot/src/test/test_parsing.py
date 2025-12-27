"""Tests for parsing module."""

import pytest
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


class TestParseJson:
    """Tests for JSON parsing."""
    
    def test_parse_object(self):
        """Test parsing JSON object."""
        result = parse_json('{"key": "value"}')
        assert result == {"key": "value"}
    
    def test_parse_array(self):
        """Test parsing JSON array."""
        result = parse_json('[1, 2, 3]')
        assert result == [1, 2, 3]
    
    def test_parse_nested(self):
        """Test parsing nested JSON."""
        result = parse_json('{"a": {"b": {"c": 1}}}')
        assert result["a"]["b"]["c"] == 1
    
    def test_parse_with_types(self):
        """Test parsing various JSON types."""
        result = parse_json('{"str": "hello", "num": 42, "float": 3.14, "bool": true, "null": null}')
        assert result["str"] == "hello"
        assert result["num"] == 42
        assert result["float"] == 3.14
        assert result["bool"] is True
        assert result["null"] is None
    
    def test_parse_invalid_raises(self):
        """Test invalid JSON raises ParseError."""
        with pytest.raises(ParseError):
            parse_json('{"invalid": }')


class TestJsonParser:
    """Tests for JsonParser class."""
    
    def test_parse_returns_result(self):
        """Test parse returns ParseResult."""
        parser = JsonParser()
        result = parser.parse('{"key": "value"}')
        
        assert isinstance(result, ParseResult)
        assert result.is_ok
    
    def test_parse_error_result(self):
        """Test parse error returns Result with error."""
        parser = JsonParser()
        result = parser.parse('{invalid}')
        
        assert result.is_err
        error = result.unwrap_err()
        assert isinstance(error, ParseError)
    
    def test_error_has_line_info(self):
        """Test error includes line information."""
        parser = JsonParser()
        result = parser.parse('{"key": invalid}')
        
        error = result.unwrap_err()
        assert error.line is not None


class TestParseResult:
    """Tests for ParseResult."""
    
    def test_is_ok(self):
        """Test is_ok property."""
        result = ParseResult(value={"data": 1})
        assert result.is_ok
        assert not result.is_err
    
    def test_is_err(self):
        """Test is_err property."""
        result = ParseResult(error=ParseError("test"))
        assert result.is_err
        assert not result.is_ok
    
    def test_unwrap_ok(self):
        """Test unwrap on Ok."""
        result = ParseResult(value="data")
        assert result.unwrap() == "data"
    
    def test_unwrap_err_raises(self):
        """Test unwrap on Err raises."""
        result = ParseResult(error=ParseError("error"))
        with pytest.raises(ParseError):
            result.unwrap()
    
    def test_unwrap_or(self):
        """Test unwrap_or with default."""
        ok_result = ParseResult(value="data")
        err_result = ParseResult(error=ParseError("error"))
        
        assert ok_result.unwrap_or("default") == "data"
        assert err_result.unwrap_or("default") == "default"


class TestParseToml:
    """Tests for TOML parsing."""
    
    def test_parse_simple(self):
        """Test parsing simple TOML."""
        result = parse_toml('key = "value"')
        assert result == {"key": "value"}
    
    def test_parse_section(self):
        """Test parsing TOML with sections."""
        result = parse_toml('[section]\nkey = "value"')
        assert result["section"]["key"] == "value"
    
    def test_parse_array(self):
        """Test parsing TOML array."""
        result = parse_toml('items = [1, 2, 3]')
        assert result["items"] == [1, 2, 3]


class TestParseYaml:
    """Tests for YAML parsing."""
    
    def test_parse_simple(self):
        """Test parsing simple YAML."""
        try:
            result = parse_yaml('key: value')
            assert result == {"key": "value"}
        except ParseError as e:
            if "pyyaml" in str(e).lower():
                pytest.skip("pyyaml not installed")
            raise
    
    def test_parse_nested(self):
        """Test parsing nested YAML."""
        try:
            result = parse_yaml('parent:\n  child: value')
            assert result["parent"]["child"] == "value"
        except ParseError as e:
            if "pyyaml" in str(e).lower():
                pytest.skip("pyyaml not installed")
            raise
    
    def test_parse_list(self):
        """Test parsing YAML list."""
        try:
            result = parse_yaml('items:\n  - one\n  - two')
            assert result["items"] == ["one", "two"]
        except ParseError as e:
            if "pyyaml" in str(e).lower():
                pytest.skip("pyyaml not installed")
            raise


class TestParseError:
    """Tests for ParseError."""
    
    def test_error_message(self):
        """Test error has message."""
        error = ParseError("Test error")
        assert "Test error" in error.message
    
    def test_error_with_location(self):
        """Test error with line/column."""
        error = ParseError("Error", line=5, column=10)
        assert error.line == 5
        assert error.column == 10
    
    def test_error_with_cause(self):
        """Test error with cause."""
        cause = ValueError("Original")
        error = ParseError("Wrapped", cause=cause)
        assert error.cause is cause
