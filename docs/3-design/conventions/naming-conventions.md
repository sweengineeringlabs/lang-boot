# Naming Conventions

Idiomatic naming patterns for each language.

---

## 🦀 Rust

| Item | Convention | Example |
|------|------------|---------|
| Variables | snake_case | `user_name`, `http_client` |
| Functions | snake_case | `get_user()`, `parse_config()` |
| Types (struct, enum) | PascalCase | `UserService`, `HttpClient` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_CONNECTIONS`, `DEFAULT_TIMEOUT` |
| Modules | snake_case | `user_service`, `http_client` |
| Crates | snake_case (or kebab-case) | `my_crate`, `my-crate` |
| Traits | PascalCase | `Serialize`, `Clone`, `IntoIterator` |
| Lifetimes | lowercase, short | `'a`, `'static` |

### Examples

```rust
const MAX_RETRIES: u32 = 3;

struct UserService {
    http_client: HttpClient,
}

impl UserService {
    fn get_user_by_id(&self, user_id: i64) -> Result<User, Error> {
        // ...
    }
}
```

---

## 🦫 Go

| Item | Convention | Example |
|------|------------|---------|
| Variables | camelCase | `userName`, `httpClient` |
| Functions | camelCase (private) | `getUser()`, `parseConfig()` |
| Functions | PascalCase (public) | `GetUser()`, `ParseConfig()` |
| Types | PascalCase | `UserService`, `HTTPClient` |
| Constants | PascalCase or camelCase | `MaxConnections`, `defaultTimeout` |
| Packages | lowercase, single word | `user`, `config`, `http` |
| Interfaces | PascalCase + "-er" suffix | `Reader`, `Writer`, `Stringer` |
| Acronyms | All caps | `HTTP`, `URL`, `ID` |

### Examples

```go
const MaxRetries = 3

type UserService struct {
    httpClient *http.Client
}

func (s *UserService) GetUserByID(userID int64) (*User, error) {
    // public (uppercase G)
}

func (s *UserService) parseResponse(resp *http.Response) error {
    // private (lowercase p)
}
```

---

## ☕ Java

| Item | Convention | Example |
|------|------------|---------|
| Variables | camelCase | `userName`, `httpClient` |
| Methods | camelCase | `getUser()`, `parseConfig()` |
| Classes | PascalCase | `UserService`, `HttpClient` |
| Interfaces | PascalCase | `UserRepository`, `Serializable` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_CONNECTIONS`, `DEFAULT_TIMEOUT` |
| Packages | lowercase, dot-separated | `com.example.service` |
| Enums | PascalCase (type), CAPS (values) | `Status.ACTIVE` |

### Examples

```java
public class UserService {
    private static final int MAX_RETRIES = 3;
    
    private final HttpClient httpClient;
    
    public User getUserById(long userId) {
        // ...
    }
    
    private Response parseResponse(HttpResponse response) {
        // ...
    }
}
```

---

## 🐍 Python

| Item | Convention | Example |
|------|------------|---------|
| Variables | snake_case | `user_name`, `http_client` |
| Functions | snake_case | `get_user()`, `parse_config()` |
| Classes | PascalCase | `UserService`, `HttpClient` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_CONNECTIONS`, `DEFAULT_TIMEOUT` |
| Modules | snake_case | `user_service.py`, `http_client.py` |
| Private | _leading_underscore | `_internal_method()` |
| "Really" private | __double_underscore | `__mangled_name` |

### Examples

```python
MAX_RETRIES = 3

class UserService:
    def __init__(self, http_client: HttpClient):
        self._http_client = http_client
    
    def get_user_by_id(self, user_id: int) -> User:
        # public
        pass
    
    def _parse_response(self, response: Response) -> dict:
        # private by convention
        pass
```

---

## Comparison

| Item | Rust | Go | Java | Python |
|------|------|-----|------|--------|
| Variables | snake_case | camelCase | camelCase | snake_case |
| Functions | snake_case | camelCase/PascalCase | camelCase | snake_case |
| Types | PascalCase | PascalCase | PascalCase | PascalCase |
| Constants | SCREAMING_SNAKE | PascalCase | SCREAMING_SNAKE | SCREAMING_SNAKE |
| Private | `pub` keyword | lowercase first | `private` keyword | `_` prefix |
