"""Tests for UUID module."""

import pytest
import re
from dev.engineeringlabs.pyboot.uuid import uuid4, uuid7, ulid, is_valid_uuid, parse_uuid


class TestUUID4:
    """Tests for UUID v4 generation."""
    
    def test_generates_valid_uuid(self):
        """Test uuid4 generates valid UUID."""
        id = uuid4()
        assert is_valid_uuid(id)
    
    def test_generates_unique(self):
        """Test uuid4 generates unique IDs."""
        ids = [uuid4() for _ in range(100)]
        assert len(set(ids)) == 100
    
    def test_correct_format(self):
        """Test uuid4 has correct format."""
        id = uuid4()
        # UUID format: 8-4-4-4-12 hex chars
        pattern = r'^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'
        assert re.match(pattern, id)


class TestUUID7:
    """Tests for UUID v7 generation."""
    
    def test_generates_valid_uuid(self):
        """Test uuid7 generates valid-looking UUID."""
        id = uuid7()
        # Should have correct format
        pattern = r'^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'
        assert re.match(pattern, id)
    
    def test_generates_unique(self):
        """Test uuid7 generates unique IDs."""
        ids = [uuid7() for _ in range(100)]
        assert len(set(ids)) == 100
    
    def test_is_sortable(self):
        """Test uuid7 IDs are sortable by generation time."""
        import time
        ids = []
        for _ in range(5):
            ids.append(uuid7())
            time.sleep(0.002)  # Small delay to ensure different timestamps
        
        # IDs should already be sorted
        assert ids == sorted(ids)


class TestULID:
    """Tests for ULID generation."""
    
    def test_generates_valid_ulid(self):
        """Test ulid generates valid ULID."""
        id = ulid()
        # ULID is 26 chars, Crockford base32
        assert len(id) == 26
        assert re.match(r'^[0-9A-HJKMNP-TV-Z]{26}$', id)
    
    def test_generates_unique(self):
        """Test ulid generates unique IDs."""
        ids = [ulid() for _ in range(100)]
        assert len(set(ids)) == 100
    
    def test_is_sortable(self):
        """Test ULIDs are sortable by generation time."""
        import time
        ids = []
        for _ in range(5):
            ids.append(ulid())
            time.sleep(0.002)  # Small delay to ensure different timestamps
        
        # IDs should already be sorted
        assert ids == sorted(ids)


class TestValidation:
    """Tests for UUID validation."""
    
    def test_valid_uuid(self):
        """Test valid UUID is recognized."""
        assert is_valid_uuid("550e8400-e29b-41d4-a716-446655440000")
    
    def test_invalid_uuid_too_short(self):
        """Test too short string is invalid."""
        assert not is_valid_uuid("550e8400-e29b-41d4")
    
    def test_invalid_uuid_bad_chars(self):
        """Test bad characters are invalid."""
        assert not is_valid_uuid("550e8400-e29b-41d4-a716-44665544GGGG")
    
    def test_invalid_uuid_no_dashes(self):
        """Test UUID without dashes is still valid."""
        # uuid.UUID accepts both formats
        assert is_valid_uuid("550e8400e29b41d4a716446655440000")
    
    def test_generated_uuid_is_valid(self):
        """Test generated UUIDs are valid."""
        for _ in range(10):
            assert is_valid_uuid(uuid4())


class TestParse:
    """Tests for UUID parsing."""
    
    def test_parse_valid_uuid(self):
        """Test parsing valid UUID."""
        parsed = parse_uuid("550e8400-e29b-41d4-a716-446655440000")
        assert parsed.version == 4
    
    def test_parse_generated_uuid(self):
        """Test parsing generated UUID."""
        id = uuid4()
        parsed = parse_uuid(id)
        assert str(parsed) == id
    
    def test_parse_invalid_raises(self):
        """Test parsing invalid string raises."""
        with pytest.raises(ValueError):
            parse_uuid("not-a-uuid")
    
    def test_parsed_uuid_properties(self):
        """Test parsed UUID has expected properties."""
        parsed = parse_uuid("550e8400-e29b-41d4-a716-446655440000")
        assert parsed.hex == "550e8400e29b41d4a716446655440000"
        assert parsed.int > 0
