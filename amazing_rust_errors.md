```
error[E0593]: closure is expected to take a single 2-tuple as argument, but it takes 2 distinct arguments
  --> src/main.rs:25:14
   |
25 |     m.iter().any( |k, v| v == k)
   |              ^^^  ------ takes 2 distinct arguments
   |              |
   |              expected closure that takes a single 2-tuple as argument
help: change the closure to accept a tuple instead of individual arguments
   |
25 |     m.iter().any( |(k, v)| v == k)
```

```
error[E0515]: cannot return value referencing temporary value
  --> src/main.rs:13:5
   |
11 |         xs.push(line.unwrap().trim())
   |                 ------------- temporary value created here
12 |     }
13 |     xs
   |     ^^ returns a value referencing data owned by the current function
```
