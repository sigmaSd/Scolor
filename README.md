# scolor

To use this branch you need to add `#![feature(const_trait_impl)]` to the top of your crate

Simple Ansi Colors (strives for ~=0 cost)
```rust
use scolor::ColorExt;
println!("{}", "hello".red().bold().underline());
println!("{}", "world".green().red_bg().italic());

use scolor::{Color, CustomStyle, ColorDesc, Effect};
const LIGHT_BLUE_ITALIC_BOLD: CustomStyle<2, 2> =
([ColorDesc::light_blue(), ColorDesc::red_bg()], [Effect::Italic, Effect::Bold]);

println!("{}", "world".custom(LIGHT_BLUE_ITALIC_BOLD));

All methods are usable in const context.

```

License: MIT
