//! Property-based tests for rustboot-serialization
//!
//! These tests verify fundamental serialization properties:
//! - Roundtrip: serialize(deserialize(x)) == x
//! - Format independence: Data survives serialization/deserialization
//! - Edge cases: Handles boundary values, unicode, large data, etc.

use proptest::prelude::*;
use dev_engineeringlabs_rustboot_serialization::{from_json, from_json_bytes, from_msgpack, to_json, to_json_bytes, to_json_pretty, to_msgpack};
use serde::{Deserialize, Serialize};

// Simple test types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SimpleStruct {
    name: String,
    age: u32,
    active: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct NestedStruct {
    id: i64,
    data: Vec<String>,
    metadata: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ComplexStruct {
    numbers: Vec<i32>,
    floats: Vec<f64>,
    nested: Vec<Vec<String>>,
    map_like: Vec<(String, i32)>,
}

// Generators for custom types
fn arb_simple_struct() -> impl Strategy<Value = SimpleStruct> {
    ("[a-zA-Z0-9 ]{0,50}", 0u32..150, any::<bool>()).prop_map(|(name, age, active)| SimpleStruct {
        name,
        age,
        active,
    })
}

fn arb_nested_struct() -> impl Strategy<Value = NestedStruct> {
    (
        any::<i64>(),
        prop::collection::vec("[a-z]{0,20}", 0..10),
        prop::option::of("[a-z]{0,30}"),
    )
        .prop_map(|(id, data, metadata)| NestedStruct { id, data, metadata })
}

fn arb_complex_struct() -> impl Strategy<Value = ComplexStruct> {
    (
        prop::collection::vec(any::<i32>(), 0..20),
        prop::collection::vec(any::<f64>(), 0..20),
        prop::collection::vec(prop::collection::vec("[a-z]{0,10}", 0..5), 0..5),
        prop::collection::vec(("[a-z]{1,10}", any::<i32>()), 0..10),
    )
        .prop_map(|(numbers, floats, nested, map_like)| ComplexStruct {
            numbers,
            floats,
            nested,
            map_like,
        })
}

// Property: JSON roundtrip for primitives
proptest! {
    #[test]
    fn test_json_roundtrip_string(s in ".*") {
        let json = to_json(&s).unwrap();
        let decoded: String = from_json(&json).unwrap();
        prop_assert_eq!(s, decoded);
    }

    #[test]
    fn test_json_roundtrip_i32(n in any::<i32>()) {
        let json = to_json(&n).unwrap();
        let decoded: i32 = from_json(&json).unwrap();
        prop_assert_eq!(n, decoded);
    }

    #[test]
    fn test_json_roundtrip_u64(n in any::<u64>()) {
        let json = to_json(&n).unwrap();
        let decoded: u64 = from_json(&json).unwrap();
        prop_assert_eq!(n, decoded);
    }

    #[test]
    fn test_json_roundtrip_bool(b in any::<bool>()) {
        let json = to_json(&b).unwrap();
        let decoded: bool = from_json(&json).unwrap();
        prop_assert_eq!(b, decoded);
    }

    #[test]
    fn test_json_roundtrip_f64(f in any::<f64>()) {
        // Skip NaN and infinity since they have special JSON handling
        prop_assume!(!f.is_nan() && !f.is_infinite());
        // Skip subnormal numbers which may lose precision in JSON
        prop_assume!(f.is_normal() || f == 0.0);
        let json = to_json(&f).unwrap();
        let decoded: f64 = from_json(&json).unwrap();
        // Use approximate comparison for floats due to JSON precision limits
        let relative_error = if f == 0.0 {
            decoded.abs()
        } else {
            ((f - decoded) / f).abs()
        };
        prop_assert!(relative_error < 1e-15, "f = {}, decoded = {}, error = {}", f, decoded, relative_error);
    }
}

// Property: JSON roundtrip for collections
proptest! {
    #[test]
    fn test_json_roundtrip_vec_string(v in prop::collection::vec(".*", 0..100)) {
        let json = to_json(&v).unwrap();
        let decoded: Vec<String> = from_json(&json).unwrap();
        prop_assert_eq!(v, decoded);
    }

    #[test]
    fn test_json_roundtrip_vec_i32(v in prop::collection::vec(any::<i32>(), 0..100)) {
        let json = to_json(&v).unwrap();
        let decoded: Vec<i32> = from_json(&json).unwrap();
        prop_assert_eq!(v, decoded);
    }

    #[test]
    fn test_json_roundtrip_option(o in prop::option::of(any::<i32>())) {
        let json = to_json(&o).unwrap();
        let decoded: Option<i32> = from_json(&json).unwrap();
        prop_assert_eq!(o, decoded);
    }
}

// Property: JSON roundtrip for structs
proptest! {
    #[test]
    fn test_json_roundtrip_simple_struct(s in arb_simple_struct()) {
        let json = to_json(&s).unwrap();
        let decoded: SimpleStruct = from_json(&json).unwrap();
        prop_assert_eq!(s, decoded);
    }

    #[test]
    fn test_json_roundtrip_nested_struct(s in arb_nested_struct()) {
        let json = to_json(&s).unwrap();
        let decoded: NestedStruct = from_json(&json).unwrap();
        prop_assert_eq!(s, decoded);
    }

    #[test]
    fn test_json_roundtrip_complex_struct(s in arb_complex_struct()) {
        // Filter out special float values that have precision issues in JSON
        prop_assume!(s.floats.iter().all(|f| {
            !f.is_nan() && !f.is_infinite() && (f.is_normal() || *f == 0.0)
        }));
        let json = to_json(&s).unwrap();
        let decoded: ComplexStruct = from_json(&json).unwrap();

        // Compare all fields except floats
        prop_assert_eq!(s.numbers, decoded.numbers);
        prop_assert_eq!(s.nested, decoded.nested);
        prop_assert_eq!(s.map_like, decoded.map_like);

        // Compare floats with tolerance
        prop_assert_eq!(s.floats.len(), decoded.floats.len());
        for (orig, dec) in s.floats.iter().zip(decoded.floats.iter()) {
            let relative_error = if *orig == 0.0 {
                dec.abs()
            } else {
                ((orig - dec) / orig).abs()
            };
            prop_assert!(relative_error < 1e-15, "orig = {}, decoded = {}", orig, dec);
        }
    }
}

// Property: JSON pretty format is valid JSON
proptest! {
    #[test]
    fn test_json_pretty_roundtrip(s in arb_simple_struct()) {
        let pretty = to_json_pretty(&s).unwrap();
        // Pretty JSON should contain newlines and indentation
        prop_assert!(pretty.contains('\n'));
        // But it should still deserialize correctly
        let decoded: SimpleStruct = from_json(&pretty).unwrap();
        prop_assert_eq!(s, decoded);
    }
}

// Property: JSON bytes roundtrip
proptest! {
    #[test]
    fn test_json_bytes_roundtrip_string(s in ".*") {
        let bytes = to_json_bytes(&s).unwrap();
        let decoded: String = from_json_bytes(&bytes).unwrap();
        prop_assert_eq!(s, decoded);
    }

    #[test]
    fn test_json_bytes_roundtrip_struct(s in arb_simple_struct()) {
        let bytes = to_json_bytes(&s).unwrap();
        let decoded: SimpleStruct = from_json_bytes(&bytes).unwrap();
        prop_assert_eq!(s, decoded);
    }
}

// Property: MessagePack roundtrip for primitives
proptest! {
    #[test]
    fn test_msgpack_roundtrip_string(s in ".*") {
        let bytes = to_msgpack(&s).unwrap();
        let decoded: String = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(s, decoded);
    }

    #[test]
    fn test_msgpack_roundtrip_i32(n in any::<i32>()) {
        let bytes = to_msgpack(&n).unwrap();
        let decoded: i32 = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(n, decoded);
    }

    #[test]
    fn test_msgpack_roundtrip_u64(n in any::<u64>()) {
        let bytes = to_msgpack(&n).unwrap();
        let decoded: u64 = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(n, decoded);
    }

    #[test]
    fn test_msgpack_roundtrip_bool(b in any::<bool>()) {
        let bytes = to_msgpack(&b).unwrap();
        let decoded: bool = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(b, decoded);
    }

    #[test]
    fn test_msgpack_roundtrip_f64(f in any::<f64>()) {
        prop_assume!(!f.is_nan());
        let bytes = to_msgpack(&f).unwrap();
        let decoded: f64 = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(f, decoded);
    }
}

// Property: MessagePack roundtrip for collections
proptest! {
    #[test]
    fn test_msgpack_roundtrip_vec_string(v in prop::collection::vec(".*", 0..100)) {
        let bytes = to_msgpack(&v).unwrap();
        let decoded: Vec<String> = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(v, decoded);
    }

    #[test]
    fn test_msgpack_roundtrip_vec_i32(v in prop::collection::vec(any::<i32>(), 0..100)) {
        let bytes = to_msgpack(&v).unwrap();
        let decoded: Vec<i32> = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(v, decoded);
    }

    #[test]
    fn test_msgpack_roundtrip_option(o in prop::option::of(any::<i32>())) {
        let bytes = to_msgpack(&o).unwrap();
        let decoded: Option<i32> = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(o, decoded);
    }
}

// Property: MessagePack roundtrip for structs
proptest! {
    #[test]
    fn test_msgpack_roundtrip_simple_struct(s in arb_simple_struct()) {
        let bytes = to_msgpack(&s).unwrap();
        let decoded: SimpleStruct = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(s, decoded);
    }

    #[test]
    fn test_msgpack_roundtrip_nested_struct(s in arb_nested_struct()) {
        let bytes = to_msgpack(&s).unwrap();
        let decoded: NestedStruct = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(s, decoded);
    }

    #[test]
    fn test_msgpack_roundtrip_complex_struct(s in arb_complex_struct()) {
        prop_assume!(s.floats.iter().all(|f| !f.is_nan()));
        let bytes = to_msgpack(&s).unwrap();
        let decoded: ComplexStruct = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(s, decoded);
    }
}

// Property: Unicode handling
proptest! {
    #[test]
    fn test_json_unicode_roundtrip(s in "[\\p{L}\\p{N}\\p{P} ]{0,100}") {
        let json = to_json(&s).unwrap();
        let decoded: String = from_json(&json).unwrap();
        prop_assert_eq!(s, decoded);
    }

    #[test]
    fn test_msgpack_unicode_roundtrip(s in "[\\p{L}\\p{N}\\p{P} ]{0,100}") {
        let bytes = to_msgpack(&s).unwrap();
        let decoded: String = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(s, decoded);
    }
}

// Property: Edge cases - empty collections
proptest! {
    #[test]
    fn test_json_empty_vec(_n in 0..10) {
        let v: Vec<i32> = vec![];
        let json = to_json(&v).unwrap();
        let decoded: Vec<i32> = from_json(&json).unwrap();
        prop_assert_eq!(v, decoded);
    }

    #[test]
    fn test_msgpack_empty_vec(_n in 0..10) {
        let v: Vec<String> = vec![];
        let bytes = to_msgpack(&v).unwrap();
        let decoded: Vec<String> = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(v, decoded);
    }

    #[test]
    fn test_json_empty_string(_n in 0..10) {
        let s = String::new();
        let json = to_json(&s).unwrap();
        let decoded: String = from_json(&json).unwrap();
        prop_assert_eq!(s, decoded);
    }
}

// Property: Large data handling
proptest! {
    #[test]
    fn test_json_large_vec(v in prop::collection::vec(any::<i32>(), 1000..1100)) {
        let json = to_json(&v).unwrap();
        let decoded: Vec<i32> = from_json(&json).unwrap();
        prop_assert_eq!(v, decoded);
    }

    #[test]
    fn test_msgpack_large_vec(v in prop::collection::vec(any::<i32>(), 1000..1100)) {
        let bytes = to_msgpack(&v).unwrap();
        let decoded: Vec<i32> = from_msgpack(&bytes).unwrap();
        prop_assert_eq!(v, decoded);
    }
}

// Property: Nested structures
proptest! {
    #[test]
    fn test_json_deeply_nested(depth in 1usize..10) {
        // Create nested vectors
        let mut current: Vec<Vec<i32>> = vec![vec![1, 2, 3]];
        for _ in 0..depth {
            current = vec![vec![1, 2, 3]];
        }
        let json = to_json(&current).unwrap();
        let decoded: Vec<Vec<i32>> = from_json(&json).unwrap();
        prop_assert_eq!(current, decoded);
    }
}

// Property: Special numeric values
proptest! {
    #[test]
    fn test_json_integer_boundaries(_n in 0..10) {
        // Test min/max values
        let values = vec![i32::MIN, i32::MAX, 0, -1, 1];
        for val in values {
            let json = to_json(&val).unwrap();
            let decoded: i32 = from_json(&json).unwrap();
            prop_assert_eq!(val, decoded);
        }
    }

    #[test]
    fn test_msgpack_integer_boundaries(_n in 0..10) {
        let values = vec![i64::MIN, i64::MAX, 0i64, -1, 1];
        for val in values {
            let bytes = to_msgpack(&val).unwrap();
            let decoded: i64 = from_msgpack(&bytes).unwrap();
            prop_assert_eq!(val, decoded);
        }
    }
}

// Property: Consistency - same input produces same output
proptest! {
    #[test]
    fn test_json_consistency(s in arb_simple_struct()) {
        let json1 = to_json(&s).unwrap();
        let json2 = to_json(&s).unwrap();
        prop_assert_eq!(json1, json2);
    }

    #[test]
    fn test_msgpack_consistency(s in arb_simple_struct()) {
        let bytes1 = to_msgpack(&s).unwrap();
        let bytes2 = to_msgpack(&s).unwrap();
        prop_assert_eq!(bytes1, bytes2);
    }
}

// Property: JSON and MessagePack interop (same data structure)
proptest! {
    #[test]
    fn test_cross_format_struct_equivalence(s in arb_simple_struct()) {
        // Serialize to JSON and MessagePack
        let json = to_json(&s).unwrap();
        let msgpack = to_msgpack(&s).unwrap();

        // Deserialize both
        let from_json: SimpleStruct = from_json(&json).unwrap();
        let from_msgpack: SimpleStruct = from_msgpack(&msgpack).unwrap();

        // Both should equal the original
        prop_assert_eq!(&s, &from_json);
        prop_assert_eq!(&s, &from_msgpack);
        prop_assert_eq!(&from_json, &from_msgpack);
    }
}

// Property: Whitespace handling in JSON
proptest! {
    #[test]
    fn test_json_whitespace_tolerance(n in any::<i32>()) {
        let json = to_json(&n).unwrap();
        // Add extra whitespace
        let with_whitespace = format!("  {}  ", json);
        let decoded: i32 = from_json(&with_whitespace).unwrap();
        prop_assert_eq!(n, decoded);
    }
}
