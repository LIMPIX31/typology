# Typology
###### Type derivation for foreign use

```rust
use typology::{Typology, type_of};

#[derive(Debug, Typology)]
struct User {
  username: String,
  age: u8,
  other: Box<[String]>
}

// Will be String
type UsernameField = type_of!(User::username);
```
