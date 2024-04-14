# Primitive data types 
## What I learnt

In Day2 of **#100daysofcode** challenge I learnt about the primitive data types in Rust.

Rust has four primary types: integers, floating-point numbers, Booleans, and characters.

### Integer Type

Intgers are of two types namely: _Signed_ and _Unsigned_.
Both types can be of 8, 16, 32, 64 and 128 bits.

#### Signed Type

- Signed type can hold both positive and negative values.
- Signed type is represented by `i` in Rust.
- Signed type can hold values from -(2<sup>n-1</sup>) to 2<sup>n-1</sup> - 1 where n is the number of bits.
- For example, i<sup>8</sup> = 256, but we can store only 255 bits as one bit represents **signed value**.

#### Unsigned Type

- Unsigned type can hold only positive values.
- Unsigned type is represented by `u` in Rust.
- Unsigned type can hold values from 0 to 2<sup>n</sup> - 1 where n is the number of bits.
- For example, u<sup>8</sup> = 256, i.e. we store numbers from 0 to 255.

### Floating-point Type

Floating-point type is represented by `f` in Rust. It can be of 32 and 64 bits.

> Note: All floating-point types are signed!!

### Boolean Type

Boolean type is represented by `bool` in Rust. It can have two values: `true` and `false`.


### Character Type

Character type is represented by `char` in Rust. It can hold a single Unicode Scalar Value which is 4 bytes in size.