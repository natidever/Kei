# Python FastAPI Class Generator

A Rust-based tool that generates FastAPI-related Python classes from annotated base classes.

## Features

- Generates `Create`, `Update`, and `Response` classes from a base class
- Supports Pydantic type annotations
- Handles optional fields and defaults
- CLI interface for easy use
- Template-based code generation

## Installation

```bash
cargo install kei
```

## Usage

```bash
kei --input ./examples --output ./generated
```

## Example

Input (`user.py`):
```python
from pydantic import BaseModel
from typing import Optional

@generate
class UserBase(BaseModel):
    id: int
    name: str
    email: str
    age: Optional[int] = None
```

Output (`user_create.py`):
```python
from pydantic import BaseModel
from typing import Optional

class UserCreate(BaseModel):
    name: str
    email: str
    age: Optional[int] = None
```

Output (`user_update.py`):
```python
from pydantic import BaseModel
from typing import Optional

class UserUpdate(BaseModel):
    name: Optional[str] = None
    email: Optional[str] = None
    age: Optional[int] = None
```

Output (`user_response.py`):
```python
from pydantic import BaseModel
from typing import Optional

class UserResponse(BaseModel):
    id: int
    name: str
    email: str
    age: Optional[int] = None
```
