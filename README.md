# scolor

Simple Ansi Colors (strives for ~=0 cost)
```rust
use scolor::ColorExt;
println!("{}", "hello".red().bold().underline());
println!("{}", "world".green().red_bg().italic());

use scolor::{Color, ColorDesc, ColorType, Effect};
const MY_COLOR: ColorDesc = ColorDesc {r: 12, g: 100, b: 200, color_type: ColorType::Fg};
println!("{}", "world".color(MY_COLOR).style(Effect::Bold));
```

License: MIT
