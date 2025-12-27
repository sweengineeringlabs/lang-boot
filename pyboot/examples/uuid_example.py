"""
PyBoot Examples - UUID Generation

Demonstrates UUID v4, v7, and ULID generation.
"""

from dev.engineeringlabs.pyboot.uuid import uuid4, uuid7, ulid, is_valid_uuid, parse_uuid


# Example 1: UUID v4 (Random)
print("=" * 50)
print("Example 1: UUID v4 (Random)")
print("=" * 50)

for i in range(5):
    id = uuid4()
    print(f"  {id}")

print()


# Example 2: UUID v7 (Time-sorted)
print("=" * 50)
print("Example 2: UUID v7 (Time-sorted)")
print("=" * 50)

print("Generating 5 UUIDs in sequence (they should be sortable):")
uuids = [uuid7() for _ in range(5)]
for id in uuids:
    print(f"  {id}")

print("\nSorted order is same as generation order:", uuids == sorted(uuids))
print()


# Example 3: ULID (Lexicographically Sortable)
print("=" * 50)
print("Example 3: ULID")
print("=" * 50)

print("Generating 5 ULIDs:")
ulids = [ulid() for _ in range(5)]
for id in ulids:
    print(f"  {id}")

print("\nULIDs are sortable:", ulids == sorted(ulids))
print()


# Example 4: UUID Validation
print("=" * 50)
print("Example 4: UUID Validation")
print("=" * 50)

test_uuids = [
    "550e8400-e29b-41d4-a716-446655440000",  # Valid
    "550e8400-e29b-41d4-a716",                # Invalid (too short)
    "not-a-uuid",                             # Invalid
    uuid4(),                                   # Valid (generated)
]

for test in test_uuids:
    valid = is_valid_uuid(test)
    print(f"  '{test}' -> {'✓ Valid' if valid else '✗ Invalid'}")
print()


# Example 5: Parse UUID
print("=" * 50)
print("Example 5: Parse UUID")
print("=" * 50)

uuid_str = "550e8400-e29b-41d4-a716-446655440000"
parsed = parse_uuid(uuid_str)
print(f"Original: {uuid_str}")
print(f"Parsed:   {parsed}")
print(f"Version:  {parsed.version}")
print(f"Variant:  {parsed.variant}")
print(f"Hex:      {parsed.hex}")
print(f"Int:      {parsed.int}")
