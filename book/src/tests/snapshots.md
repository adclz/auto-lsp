# Snapshots

Since all AST nodes implement the `Debug` trait, it is very easy to take snapshots of them.

Therefore, you can use any snapshot testing library such as [`insta`](https://docs.rs/insta/latest/insta/) or [`expect_test`](https://docs.rs/expect-test/latest/expect_test/).


```admonish
Since Some errors during parsing might be silent, it is recommended to have an utility fn/macro to create a Database and use `get_ast` with an accumulator to check for errors.
```
