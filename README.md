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

For even more zero cost power you can enable `zero-cost` feature

It makes the generated ASCII code as optimal as it can be

But the cost is that it's less ergonomic, the API is invoked like this:
```rust
println!("{}", "hello".green().bold::<1>().red_bg::<2>().italic::<2>());
```

License: MIT
