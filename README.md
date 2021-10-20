# scolor

Simple ansi colors
```rust
use scolor::{Color, ColorExt};

println!("{}", "hello".red().bold().underline());
println!("{}", "world".green().red_bg().italic());
println!("{}", "!".rgb_fg(123, 12, 50));
```

License: MIT
