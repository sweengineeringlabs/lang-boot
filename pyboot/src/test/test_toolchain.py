"""Tests for toolchain module."""

import pytest
import os
from dev.engineeringlabs.pyboot.toolchain import (
    get_version,
    get_python_version,
    get_environment,
    is_debug,
    is_production,
    get_build_info,
    Environment,
    BuildMode,
)


class TestGetVersion:
    """Tests for get_version function."""
    
    def test_returns_string(self):
        """Test version is a string."""
        version = get_version()
        assert isinstance(version, str)
    
    def test_version_format(self):
        """Test version has expected format."""
        version = get_version()
        # Should be semver-like or at least not empty
        assert len(version) > 0


class TestGetPythonVersion:
    """Tests for get_python_version function."""
    
    def test_returns_string(self):
        """Test Python version is a string."""
        version = get_python_version()
        assert isinstance(version, str)
    
    def test_version_format(self):
        """Test version has major.minor.patch format."""
        version = get_python_version()
        parts = version.split(".")
        assert len(parts) >= 2
        assert all(part.isdigit() for part in parts[:2])


class TestGetEnvironment:
    """Tests for get_environment function."""
    
    def test_returns_environment(self):
        """Test returns Environment enum."""
        env = get_environment()
        assert isinstance(env, Environment)
    
    def test_respects_env_var(self):
        """Test respects ENV environment variable."""
        original = os.environ.get("ENV")
        try:
            os.environ["ENV"] = "production"
            assert get_environment() == Environment.PRODUCTION
            
            os.environ["ENV"] = "staging"
            assert get_environment() == Environment.STAGING
            
            os.environ["ENV"] = "test"
            assert get_environment() == Environment.TEST
            
            os.environ["ENV"] = "development"
            assert get_environment() == Environment.DEVELOPMENT
        finally:
            if original:
                os.environ["ENV"] = original
            elif "ENV" in os.environ:
                del os.environ["ENV"]


class TestIsDebug:
    """Tests for is_debug function."""
    
    def test_returns_bool(self):
        """Test returns boolean."""
        assert isinstance(is_debug(), bool)
    
    def test_matches_debug_flag(self):
        """Test matches Python's __debug__ flag."""
        assert is_debug() == __debug__


class TestIsProduction:
    """Tests for is_production function."""
    
    def test_returns_bool(self):
        """Test returns boolean."""
        assert isinstance(is_production(), bool)


class TestGetBuildInfo:
    """Tests for get_build_info function."""
    
    def test_returns_build_info(self):
        """Test returns BuildInfo object."""
        info = get_build_info()
        assert hasattr(info, "version")
        assert hasattr(info, "python_version")
        assert hasattr(info, "platform")
        assert hasattr(info, "environment")
        assert hasattr(info, "build_mode")
        assert hasattr(info, "timestamp")
    
    def test_environment_is_enum(self):
        """Test environment is Environment enum."""
        info = get_build_info()
        assert isinstance(info.environment, Environment)
    
    def test_build_mode_is_enum(self):
        """Test build_mode is BuildMode enum."""
        info = get_build_info()
        assert isinstance(info.build_mode, BuildMode)
    
    def test_timestamp_is_iso_format(self):
        """Test timestamp is ISO format."""
        info = get_build_info()
        # Should contain 'T' for ISO format
        assert "T" in info.timestamp or "-" in info.timestamp


class TestEnvironmentEnum:
    """Tests for Environment enum."""
    
    def test_all_environments_exist(self):
        """Test all expected environments exist."""
        assert Environment.DEVELOPMENT
        assert Environment.STAGING
        assert Environment.PRODUCTION
        assert Environment.TEST


class TestBuildModeEnum:
    """Tests for BuildMode enum."""
    
    def test_all_modes_exist(self):
        """Test all expected modes exist."""
        assert BuildMode.DEBUG
        assert BuildMode.RELEASE
