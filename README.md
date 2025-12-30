# J64ID Generator in Rust

**J**ust **64** bit **ID** Generator

- Based on timestamp and randomly seeded counter
- Roughly ordered by timestamp

## Implementation

```
|                     |                         |                    |
| 40-bit              | 12-bit                  | 12-bit             |
| timestamp + counter | randomly seeded counter | randomly generated |
```
