# Module & Import Conventions

Idiomatic module and import patterns for each language.

---

## 🦀 Rust

### Module System

```rust
// src/lib.rs
mod user;           // Loads user.rs or user/mod.rs
pub mod api;        // Public module

pub use user::User; // Re-export

// src/user.rs
pub struct User { ... }
```

### Imports

```rust
use std::collections::HashMap;
use crate::user::User;           // From crate root
use super::sibling::Item;         // From parent module
use self::submodule::Thing;       // From current module

// Multiple from same path
use std::io::{Read, Write, BufReader};

// Alias
use std::collections::HashMap as Map;
```

### Conventions

- One `mod` declaration per module
- Use `pub use` for re-exports (facade pattern)
- Group imports: std, external crates, crate
- Avoid `use super::*` (pollutes namespace)

---

## 🦫 Go

### Package System

```go
// user/user.go
package user

type User struct { ... }

func New() *User { ... }
```

### Imports

```go
import (
    "fmt"                           // Standard library
    "encoding/json"
    
    "github.com/gin-gonic/gin"      // External
    
    "myproject/internal/database"   // Internal
    "myproject/pkg/util"
)

// Alias
import (
    log "github.com/sirupsen/logrus"
)

// Blank import (side effects only)
import _ "github.com/lib/pq"
```

### Conventions

- One package per directory
- Package name = directory name (usually)
- Group imports: stdlib, external, internal
- Use `goimports` for automatic formatting
- No circular imports allowed

---

## ☕ Java

### Package System

```java
// src/main/java/com/example/user/User.java
package com.example.user;

public class User { ... }
```

### Imports

```java
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import com.example.user.User;
import com.example.service.UserService;

// Static import
import static java.util.stream.Collectors.toList;

// Wildcard (discouraged)
import java.util.*;
```

### Conventions

- Package hierarchy matches directory structure
- One public class per file
- File name = class name
- Group: java.\*, javax.\*, external, project
- Avoid wildcard imports

---

## 🐍 Python

### Module System

```python
# mypackage/user.py
class User:
    ...

# mypackage/__init__.py
from .user import User  # Re-export
```

### Imports

```python
# Standard library
import os
from pathlib import Path

# Third-party
import requests
from pydantic import BaseModel

# Local
from mypackage.user import User
from . import utils                # Relative
from ..sibling import helper       # Parent relative

# Alias
import numpy as np
```

### Conventions

- Group: stdlib, third-party, local
- Use absolute imports (preferred over relative)
- Import modules, not just classes
- Avoid `from x import *`
- Use `__all__` to define public API

```python
# mypackage/__init__.py
__all__ = ["User", "UserService"]
```

---

## Comparison

| Aspect | Rust | Go | Java | Python |
|--------|------|-----|------|--------|
| Unit | Module | Package | Package | Module |
| Declaration | `mod name;` | `package name` | `package path;` | (file-based) |
| Import | `use path::Item` | `import "path"` | `import path.Class` | `from x import y` |
| Re-export | `pub use` | N/A | N/A | `__init__.py` |
| Circular | Allowed (limited) | ❌ Forbidden | ✅ Allowed | ✅ Allowed |

---

## Import Order Convention

All languages follow similar grouping:

```
1. Standard library
2. Third-party/external
3. Local/project imports
```

Separate groups with blank lines.
