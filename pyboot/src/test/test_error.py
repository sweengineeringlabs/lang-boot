"""Tests for error module."""

import pytest
from dev.engineeringlabs.pyboot.error import Result, Ok, Err, ErrorCode, PybootError, chain_errors, wrap_error


class TestResult:
    """Tests for Result monad."""
    
    def test_ok_is_ok(self):
        """Test Ok returns is_ok=True."""
        result = Ok(42)
        assert result.is_ok
        assert not result.is_err
    
    def test_err_is_err(self):
        """Test Err returns is_err=True."""
        result = Err("error")
        assert result.is_err
        assert not result.is_ok
    
    def test_unwrap_ok(self):
        """Test unwrap on Ok returns value."""
        result = Ok(42)
        assert result.unwrap() == 42
    
    def test_unwrap_err_raises(self):
        """Test unwrap on Err raises ValueError."""
        result = Err("error")
        with pytest.raises(ValueError):
            result.unwrap()
    
    def test_unwrap_err(self):
        """Test unwrap_err on Err returns error."""
        result = Err("error message")
        assert result.unwrap_err() == "error message"
    
    def test_unwrap_err_on_ok_raises(self):
        """Test unwrap_err on Ok raises ValueError."""
        result = Ok(42)
        with pytest.raises(ValueError):
            result.unwrap_err()
    
    def test_unwrap_or(self):
        """Test unwrap_or with default."""
        assert Ok(42).unwrap_or(0) == 42
        assert Err("error").unwrap_or(0) == 0
    
    def test_map_ok(self):
        """Test map on Ok transforms value."""
        result = Ok(5)
        mapped = result.map(lambda x: x * 2)
        assert mapped.unwrap() == 10
    
    def test_map_err(self):
        """Test map on Err passes through."""
        result: Result[int, str] = Err("error")
        mapped = result.map(lambda x: x * 2)
        assert mapped.is_err
        assert mapped.unwrap_err() == "error"
    
    def test_map_err_ok(self):
        """Test map_err on Ok passes through."""
        result = Ok(42)
        mapped = result.map_err(lambda e: f"wrapped: {e}")
        assert mapped.is_ok
        assert mapped.unwrap() == 42
    
    def test_map_err_err(self):
        """Test map_err on Err transforms error."""
        result: Result[int, str] = Err("original")
        mapped = result.map_err(lambda e: f"wrapped: {e}")
        assert mapped.is_err
        assert mapped.unwrap_err() == "wrapped: original"
    
    def test_and_then_ok(self):
        """Test and_then chains Result-returning functions."""
        def double(x: int) -> Result[int, str]:
            return Ok(x * 2)
        
        result = Ok(5).and_then(double)
        assert result.unwrap() == 10
    
    def test_and_then_err(self):
        """Test and_then short-circuits on Err."""
        def double(x: int) -> Result[int, str]:
            return Ok(x * 2)
        
        result: Result[int, str] = Err("error")
        mapped = result.and_then(double)
        assert mapped.is_err


class TestPybootError:
    """Tests for PybootError."""
    
    def test_basic_error(self):
        """Test basic error creation."""
        error = PybootError("Something went wrong")
        assert error.message == "Something went wrong"
        assert error.code == ErrorCode.UNKNOWN
    
    def test_error_with_code(self):
        """Test error with specific code."""
        error = PybootError("Not found", code=ErrorCode.NOT_FOUND)
        assert error.code == ErrorCode.NOT_FOUND
    
    def test_error_with_cause(self):
        """Test error with cause."""
        cause = ValueError("original")
        error = PybootError("Wrapped", cause=cause)
        assert error.cause is cause
    
    def test_error_with_details(self):
        """Test error with details."""
        error = PybootError(
            "Validation failed",
            code=ErrorCode.VALIDATION,
            details={"field": "email", "reason": "invalid format"},
        )
        assert error.details["field"] == "email"
    
    def test_error_str(self):
        """Test error string representation."""
        error = PybootError("Test error", code=ErrorCode.NOT_FOUND)
        assert "[NOT_FOUND]" in str(error)
        assert "Test error" in str(error)
    
    def test_error_to_dict(self):
        """Test error serialization."""
        error = PybootError(
            "Test",
            code=ErrorCode.VALIDATION,
            details={"key": "value"},
        )
        d = error.to_dict()
        assert d["code"] == "VALIDATION"
        assert d["message"] == "Test"
        assert d["details"]["key"] == "value"


class TestErrorUtilities:
    """Tests for error utilities."""
    
    def test_chain_errors(self):
        """Test chaining errors."""
        e1 = ValueError("first")
        e2 = TypeError("second")
        e3 = RuntimeError("third")
        
        chained = chain_errors(e1, e2, e3)
        
        assert isinstance(chained, ValueError)
        assert chained.__cause__.__cause__ is e3
    
    def test_wrap_error(self):
        """Test wrapping errors."""
        original = IOError("File not found")
        wrapped = wrap_error(original, PybootError, "Operation failed")
        
        assert isinstance(wrapped, PybootError)
        assert wrapped.__cause__ is original
        assert "Operation failed" in str(wrapped)


class TestErrorCode:
    """Tests for ErrorCode enum."""
    
    def test_all_codes_exist(self):
        """Test expected error codes exist."""
        assert ErrorCode.UNKNOWN
        assert ErrorCode.INTERNAL
        assert ErrorCode.VALIDATION
        assert ErrorCode.NOT_FOUND
        assert ErrorCode.UNAUTHORIZED
        assert ErrorCode.TIMEOUT
