#!/bin/bash
# Test script for the Rustboot Full Application Example

set -e

BASE_URL="http://localhost:3000"

echo "================================"
echo "Rustboot TODO API Test Script"
echo "================================"
echo ""

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}1. Testing API root endpoint...${NC}"
curl -s "$BASE_URL/" | jq '.'
echo ""

echo -e "${BLUE}2. Testing health check endpoint...${NC}"
curl -s "$BASE_URL/health" | jq '.'
echo ""

echo -e "${BLUE}3. Registering a new user...${NC}"
REGISTER_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "SecurePass123"
  }')
echo "$REGISTER_RESPONSE" | jq '.'
USER_ID=$(echo "$REGISTER_RESPONSE" | jq -r '.data.id')
echo -e "${GREEN}Created user with ID: $USER_ID${NC}"
echo ""

echo -e "${BLUE}4. Logging in...${NC}"
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "SecurePass123"
  }')
echo "$LOGIN_RESPONSE" | jq '.'
SESSION_ID=$(echo "$LOGIN_RESPONSE" | jq -r '.data.session_id')
echo -e "${GREEN}Login successful. Session ID: $SESSION_ID${NC}"
echo ""

echo -e "${BLUE}5. Creating a public TODO (without authentication)...${NC}"
PUBLIC_TODO_RESPONSE=$(curl -s -X POST "$BASE_URL/api/todos" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Buy groceries",
    "description": "Milk, bread, eggs"
  }')
echo "$PUBLIC_TODO_RESPONSE" | jq '.'
PUBLIC_TODO_ID=$(echo "$PUBLIC_TODO_RESPONSE" | jq -r '.data.id')
echo -e "${GREEN}Created public TODO with ID: $PUBLIC_TODO_ID${NC}"
echo ""

echo -e "${BLUE}6. Creating a user TODO (with authentication)...${NC}"
USER_TODO_RESPONSE=$(curl -s -X POST "$BASE_URL/api/todos" \
  -H "Content-Type: application/json" \
  -H "Cookie: todo_session=$SESSION_ID" \
  -d '{
    "title": "Complete Rustboot example",
    "description": "Finish the full application example"
  }')
echo "$USER_TODO_RESPONSE" | jq '.'
USER_TODO_ID=$(echo "$USER_TODO_RESPONSE" | jq -r '.data.id')
echo -e "${GREEN}Created user TODO with ID: $USER_TODO_ID${NC}"
echo ""

echo -e "${BLUE}7. Listing public TODOs (without authentication)...${NC}"
curl -s "$BASE_URL/api/todos" | jq '.'
echo ""

echo -e "${BLUE}8. Listing user TODOs (with authentication)...${NC}"
curl -s "$BASE_URL/api/todos" \
  -H "Cookie: todo_session=$SESSION_ID" | jq '.'
echo ""

echo -e "${BLUE}9. Getting a specific TODO...${NC}"
curl -s "$BASE_URL/api/todos/$PUBLIC_TODO_ID" | jq '.'
echo ""

echo -e "${BLUE}10. Updating a TODO...${NC}"
curl -s -X PUT "$BASE_URL/api/todos/$PUBLIC_TODO_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "completed": true
  }' | jq '.'
echo ""

echo -e "${BLUE}11. Verifying the update...${NC}"
curl -s "$BASE_URL/api/todos/$PUBLIC_TODO_ID" | jq '.'
echo ""

echo -e "${BLUE}12. Deleting a TODO...${NC}"
curl -s -X DELETE "$BASE_URL/api/todos/$PUBLIC_TODO_ID" | jq '.'
echo ""

echo -e "${BLUE}13. Verifying deletion (should return 404)...${NC}"
curl -s "$BASE_URL/api/todos/$PUBLIC_TODO_ID" | jq '.'
echo ""

echo -e "${GREEN}================================${NC}"
echo -e "${GREEN}All tests completed successfully!${NC}"
echo -e "${GREEN}================================${NC}"
