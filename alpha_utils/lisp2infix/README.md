# s-converter

## Usage

convert common expression to s-expression

```bash
cargo build
./target/debug/s-converter "(! (shift[1] * (a[77] * shift[0])))" "lisp"
```

convert s-expression to common expression

```bash
cargo build
./target/debug/s-converter "(! (* shift[1] (* a[77] shift[0])))" 
```

split concat: input s-expression

## Note:

* `!Var` and `!Expr` is not allow in infix expression ( `!a` and `!(!a)` is not allow), you should have space between `!` and `Var` or `Expr` ( `! a` and `! (! a)` is allow)
* `*` and `+` adn `&` only allow two arguments ( `a * b * c` is not allow, you should use `a * (b * c)` instead)
* ~~`!` shall be warp by `()` ( `!a * b` is not allow, you should use `(!a) * b` instead)~~ Now support
* All expr should be warp by `()` ( `a * b` is not allow, you should use `(a * b)` instead)

