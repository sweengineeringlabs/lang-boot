// Example demonstrating OpenAPI utilities in goboot.
package main

import (
	"encoding/json"
	"fmt"

	"dev.engineeringlabs/goboot/openapi"
)

// User represents a user in the system.
type User struct {
	ID    int64  `json:"id"`
	Name  string `json:"name"`
	Email string `json:"email"`
}

// CreateUserRequest represents a request to create a user.
type CreateUserRequest struct {
	Name  string `json:"name"`
	Email string `json:"email"`
}

func main() {
	// Create a new OpenAPI specification
	spec := openapi.NewSpec("User API", "1.0.0")
	spec.Info.Description = "API for managing users"

	// Add paths
	spec.Paths["/users"] = openapi.Path{
		Summary: "User operations",
		Get: &openapi.Operation{
			Summary:     "List all users",
			OperationID: "listUsers",
			Tags:        []string{"users"},
			Responses: map[string]openapi.Response{
				"200": {
					Description: "Successful operation",
					Content: map[string]openapi.MediaType{
						"application/json": {
							Schema: openapi.ArrayOf(User{}),
						},
					},
				},
			},
		},
		Post: &openapi.Operation{
			Summary:     "Create a user",
			OperationID: "createUser",
			Tags:        []string{"users"},
			RequestBody: &openapi.RequestBody{
				Required: true,
				Content: map[string]openapi.MediaType{
					"application/json": {
						Schema: openapi.SchemaFrom(CreateUserRequest{}),
					},
				},
			},
			Responses: map[string]openapi.Response{
				"201": {
					Description: "User created",
					Content: map[string]openapi.MediaType{
						"application/json": {
							Schema: openapi.SchemaFrom(User{}),
						},
					},
				},
			},
		},
	}

	spec.Paths["/users/{id}"] = openapi.Path{
		Summary: "Single user operations",
		Parameters: []openapi.Parameter{
			{
				Name:        "id",
				In:          "path",
				Required:    true,
				Description: "User ID",
				Schema:      &openapi.Schema{Type: "integer", Format: "int64"},
			},
		},
		Get: &openapi.Operation{
			Summary:     "Get user by ID",
			OperationID: "getUserById",
			Tags:        []string{"users"},
			Responses: map[string]openapi.Response{
				"200": {
					Description: "Successful operation",
					Content: map[string]openapi.MediaType{
						"application/json": {
							Schema: openapi.SchemaFrom(User{}),
						},
					},
				},
				"404": {
					Description: "User not found",
				},
			},
		},
		Delete: &openapi.Operation{
			Summary:     "Delete user",
			OperationID: "deleteUser",
			Tags:        []string{"users"},
			Responses: map[string]openapi.Response{
				"204": {Description: "User deleted"},
				"404": {Description: "User not found"},
			},
		},
	}

	// Add tags
	spec.Tags = []openapi.Tag{
		{Name: "users", Description: "User management operations"},
	}

	// Output as JSON
	output, err := json.MarshalIndent(spec, "", "  ")
	if err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}

	fmt.Println(string(output))
}
