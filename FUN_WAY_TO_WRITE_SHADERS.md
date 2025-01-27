do it like this:

```rust
let mut entrypoint = None;
let mut exitpoint = None;
let shd = shader::build_vsh(|mut b| {
    let inpos = shader::v(0)?;
    let inclr = shader::v(1)?;
    let outpos = b.o(0,shader::Position.xyzw())?;
    let outclr = b.o(1,shader::Color.xyzw())?;
    b + shader::label(&mut entrypoint)
      + shader::mov(outpos, inpos)
      + shader::mov(outclr, inclr)
      + shader::label(&mut exitpoint)
});

let mut enc = CommandEncoder::new();
enc = enc
  + shd.load()
  + shd.outmap()
  + shd.num_attr()
  + immediate::mode(vertex_making_function_goes_here);
queue::submit(enc);
```
