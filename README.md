# scolor

Simple Ansi Colors (strives for ~=0 cost)
```rust
use scolor::ColorExt;
println!("{}", "hello".red().bold().underline());
println!("{}", "world".green().red_bg().italic());

use scolor::{Color, CustomStyle, ColorDesc, Effect};
const LIGHT_BLUE_ITALIC_BOLD: CustomStyle<2, 2> =
([ColorDesc::light_blue(), ColorDesc::red_bg()], [Effect::Italic, Effect::Bold]);

println!("{}", "world".custom(LIGHT_BLUE_ITALIC_BOLD));
```

Const equivalent of trait functions are provided as freestanding top-level functions
```rust
const BLUE_WORLD: scolor::ColorFmt<'_,str,1,0> = scolor::blue("world");
```

For even more zero cost power you can enable `zero-cost` feature

It makes the generated ASCII code as optimal as it can be

But the cost is that it's less ergonomic, the API is invoked like this:
```rust
use scolor::ColorExt;
println!("{}", "hello".green().bold::<1>().red_bg::<2>().italic::<2>());

use scolor::{ColorDesc, ColorFmt, Effect, green};
const _:() = {
    let fmt = green("hello").italic::<1>().bold::<2>().red_bg::<2>().crossed_out::<3>();
    assert!(matches!(ColorFmt{fmt:"hello",color:[ColorDesc::green(),ColorDesc::red_bg()],effect:[Effect::Italic, Effect::Bold]}, fmt));
};
```

For an even better API you can use the nightly branch <https://github.com/sigmaSd/Scolor/tree/nightly>

License: MIT
