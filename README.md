# scolor

Simple Ansi Colors (strives for ~=0 cost)
```rust
use scolor::ColorExt;

println!("{}", "hello".red().bold().underline());
println!("{}", "world".green().red_bg().italic());
```

License: MIT
