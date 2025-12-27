//! API Request Validation Example
//!
//! Demonstrates validation for REST API request payloads

use dev_engineeringlabs_rustboot_validation::*;

#[derive(Debug)]
struct CreateProductRequest {
    name: String,
    description: String,
    price: f64,
    quantity: i32,
    category: String,
    tags: Vec<String>,
}

fn main() {
    println!("=== Rustboot Validation Example: API Request Validation ===\n");

    // Valid product
    let valid_product = CreateProductRequest {
        name: "Wireless Mouse".to_string(),
        description: "Ergonomic wireless mouse with USB receiver".to_string(),
        price: 29.99,
        quantity: 150,
        category: "electronics".to_string(),
        tags: vec!["wireless".to_string(), "mouse".to_string()],
    };

    validate_product_request(&valid_product);

    println!("\n---\n");

    // Invalid product
    let invalid_product = CreateProductRequest {
        name: "".to_string(),
        description: "A".to_string(),
        price: -10.0,
        quantity: 0,
        category: "INVALID CATEGORY WITH SPACES".to_string(),
        tags: vec![],
    };

    validate_product_request(&invalid_product);
}

fn validate_product_request(product: &CreateProductRequest) {
    println!("Validating product request: {:?}\n", product.name);

    // Product name validation
    let name_validator = StringValidationBuilder::new("name")
        .not_empty()
        .min_length(3)
        .max_length(100)
        .build();

    match name_validator.validate(&product.name) {
        Ok(_) => println!("✓ Product name is valid"),
        Err(errors) => println!("✗ Product name validation failed: {:?}", errors),
    }

    // Description validation
    let description_validator = StringValidationBuilder::new("description")
        .not_empty()
        .min_length(10)
        .max_length(500)
        .build();

    match description_validator.validate(&product.description) {
        Ok(_) => println!("✓ Description is valid"),
        Err(errors) => println!("✗ Description validation failed: {:?}", errors),
    }

    // Price validation
    let price_validator = NumericValidationBuilder::new("price")
        .min(0.01)
        .max(999999.99)
        .build();

    match price_validator.validate(&product.price) {
        Ok(_) => println!("✓ Price is valid"),
        Err(errors) => println!("✗ Price validation failed: {:?}", errors),
    }

    // Quantity validation
    let quantity_validator = NumericValidationBuilder::new("quantity")
        .min(1)
        .max(100000)
        .build();

    match quantity_validator.validate(&product.quantity) {
        Ok(_) => println!("✓ Quantity is valid"),
        Err(errors) => println!("✗ Quantity validation failed: {:?}", errors),
    }

    // Category validation (slug format)
    let category_validator = StringValidationBuilder::new("category")
        .not_empty()
        .min_length(2)
        .max_length(50)
        .matches(
            |s: &String| s.chars().all(|c| c.is_lowercase() || c == '-'),
            "Category must be lowercase with hyphens only",
        )
        .build();

    match category_validator.validate(&product.category) {
        Ok(_) => println!("✓ Category is valid"),
        Err(errors) => println!("✗ Category validation failed: {:?}", errors),
    }

    // Tags validation
    if product.tags.is_empty() {
        println!("✗ Tags cannot be empty (business rule)");
    } else {
        println!("✓ Tags provided: {} tags", product.tags.len());
        
        // Validate each tag
        let tag_validator = StringValidationBuilder::new("tag")
            .not_empty()
            .min_length(2)
            .max_length(20)
            .matches(
                |s: &String| s.chars().all(|c| c.is_alphanumeric() || c == '-'),
                "Tag must be alphanumeric with hyphens only",
            )
            .build();

        for (i, tag) in product.tags.iter().enumerate() {
            match tag_validator.validate(tag) {
                Ok(_) => println!("  ✓ Tag {}: {}", i + 1, tag),
                Err(_) => println!("  ✗ Tag {}: {} is invalid", i + 1, tag),
            }
        }
    }
}
