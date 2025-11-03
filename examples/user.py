from pydantic import BaseModel
from typing import Optional

@generate
class UserBase(BaseModel):
    id: int
    name: str
    email: str
    age: Optional[int] = None
