# scolor

Simple Ansi Colors (strives for ~=0 cost)
```rust
use scolor::ColorExt;
println!("{}", "hello".red().bold().underline());
println!("{}", "world".green().red_bg().italic());

use scolor::{Color, Rgb, ColorType};
const MY_COLOR: Rgb = Rgb {r: 12, g: 100, b: 200, ctype: ColorType::Fg};
println!("{}", "world".rgb(MY_COLOR).bold());
```

License: MIT
