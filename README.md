# Rust

## mut v.s. shadowing

- shadowing is creating a new variable and reusing the same name.

## Data types

- default type
  - integer: i32
  - float: f64

- char: 4 bytes

- array: (in stack)
  - `let a: [i32; 5] = [1, 2, 3, 4, 5]`
  - `let a = [1, 2, 3, 4, 5]`
  - `let a = [3; 5]` = `let a = [3, 3, 3, 3, 3]` (repeated)

## Function

- function
  - statements do not return values: `let x = (let y = 6);` (error)
    - `let y = 6` statement does not return a value.
    - `x = y = 10` does not work in Rust.
  - return the result of expression ==without ending semicolons==

    ```rust
    {
        let x = 3;
        x + 1
    }
    ```

    - return `x+1`
    - return `()` (`Unit` in Scala, ==empty tuple== in Rust) with ending semicolons (`;`)

## Control

- if can return a value. like `? :`

```rust
let number = if condition {
  5
} else {
  6
};
```

- loop can return a value (`loop break`)

```rust
let result = loop {
    count += 1;

    if counter == 10 {
        break counter * 2;
    }
};
```

## Stack v.s. Heap

| Stack | Heap |
| - | - |
| LIFO | x |
| Fixed Size ^[fixed_size: data with unknown size at compile time or size might change must be stored in Heap instead.] | allocating a big enough space and return a ==pointer==^[pointer is fixed size and can be stored in Stack] |

## Ownership

- Variable is the owner of a value in Rust.
- Only one owner at a time for a value.
- Value will be dropped when owner is out of scope.

### Ways Variables and Data Interact

- Copy
  - Scalar types: integer, floating-point, boolean, character
    - fixed size known on compile time
  - tuple of scalar types
  - `Copy` trait
    - Types only on Stack
    - must implement `Clone` trait
- Move
  - unknown data size, like `String`
  - avoid double free
- Clone
  - copy heap data deeply

#### Transfer Ownership

- value passing into function
- get value from function returning.
- when a variable that including data on Heap goes out of scope, the value will be cleaned up by `drop` unless the data has been moved to be owned by another variable.

## Reference and Borrowing

- `&`: ==*referencing*==
- `*`: ==*dereference*==

> We call having references as function parameters borrowing

- as function parameters
- get reference from variable (`&`)

### mut reference

- only one mutable reference to a data in a particular scope.
  - avoid data racing.
- can't borrow mutable and 

> Even though borrowing errors may be ==frustrating== at times, remember that it’s the Rust compiler pointing out a potential bug early (at compile time rather than at runtime) and showing you exactly where the problem is. Then you don’t have to track down why your data isn’t what you thought it was.