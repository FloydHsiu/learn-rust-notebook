# Review of `q0002_rotate_string`

Here is a comprehensive review of your initial draft for the "Rotate String" problem.

Your initial draft is excellent! It uses a well-known, clever, and efficient algorithm to solve the problem. The review will focus on a key idiomatic refinement related to Rust's type system and API design.

## 1. The Rustacean Way: Idiomatic Refinements

Your algorithm is already top-notch. The main area for an idiomatic improvement is in the function's signature.

```rust
// Your Draft (simplified)
pub fn rotate_string(s: String, goal: String) -> bool { ... }

// Idiomatic Version
pub fn rotate_string(s: &str, goal: &str) -> bool { ... }
```

By changing the parameters from the owned `String` type to the borrowed `&str` (string slice) type, we make the function more flexible and efficient. It signals that the function only needs to *read* the string data, not take ownership of it. This is a fundamental concept in Rust: **borrowing over owning** when you don't need to modify or consume the data.

Here is the fully refined, idiomatic version:

```rust
pub fn rotate_string(s: &str, goal: &str) -> bool {
    // The length check is a crucial first step for both correctness and efficiency.
    // The concatenation and `contains` check are only performed if the lengths match.
    s.len() == goal.len() && (s.to_owned() + s).contains(goal)
}
```

**Key Changes & Idioms:**

- **`&str` over `String`:** As discussed, this makes the API more versatile. It can now accept `String` objects (e.g., `rotate_string(&my_string, &goal_string)`), string literals (e.g., `rotate_string("abc", "bca")`), and other string slices without requiring the caller to clone data unnecessarily.
- **Concise Logic:** The boolean logic `A && B` is very clear. It first checks the length (`A`), and only if that's true does it proceed to the more expensive concatenation and search (`B`).
- **`.to_owned() + &str`:** This is a clean way to perform the concatenation. `s.to_owned()` creates a new `String` from the `&str` slice `s`, and then the `+` operator appends the second `&str` slice `s` to it.

## 2. Algorithm & Complexity Analysis

- **Algorithm:** String Concatenation and Search. The core idea is that if `goal` is a rotation of `s`, it must be a substring of `s` concatenated with itself. The length check is vital to ensure it's a valid rotation.
- **Time Complexity:** `O(N)`, where `N` is the length of the strings.
  - `s.len() == goal.len()` is `O(1)`.
  - `s.to_owned() + s` allocates and creates a new string of length `2N`, which is an `O(N)` operation.
  - `.contains(goal)` uses efficient string searching algorithms, which on average perform in `O(N)` time (length of haystack + length of needle).
- **Space Complexity:** `O(N)`. This is the main trade-off of this algorithm. A new string of length `2N` is temporarily created in memory. For most competitive programming and general use cases, this is perfectly acceptable.

## 3. Blind Spot Detection

Your draft was logically flawless. You correctly identified the two necessary conditions:

1. The lengths must be equal.
2. The `goal` must be contained in the doubled `s` string.

Your inclusion of `s.len() == goal.len()` prevents false positives for cases like `s = "aa"`, `goal = "a"`, where `goal` is a substring of `s+s` ("aaaa") but not a rotation. There are no edge cases missed.

## 4. The "Why": Benefits of the Idiomatic Approach

1. **API Design & Flexibility:** Using `&str` creates a better API. A function should ask for the minimum it needs. Since this function only reads data, it should borrow (`&str`) rather than own (`String`). This prevents the caller from having to give up ownership of their strings or perform a `.clone()` just to call your function.
2. **Clarity of Intent:** The signature `fn(&str, &str)` immediately tells a Rust programmer that the function is non-destructive and will not consume the input strings.
3. **Efficiency:** While your algorithm was already efficient, using `&str` prevents potential overhead at the call site, making the overall program more performant.

## 5. Key Rust Concept

- **`String` vs. `&str`:** The core lesson here is the distinction between the heap-allocated, owned, mutable `String` and the fixed-size, borrowed, immutable-by-default view into a string, `&str`.
- **Borrowing:** The practice of passing references (`&`) to data instead of transferring ownership. It's a cornerstone of Rust's memory safety guarantees without needing a garbage collector.
- **Method Chaining:** The concise `s.len() == goal.len() && ...` demonstrates how Rust encourages chaining methods and expressions for clean, readable logic.
