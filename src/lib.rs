#![crate_id = "static_mdo#0.0.1"]
#![feature(macro_rules)]

//! static_mdo
/* monadic do notation without using closures.
 * a set of rust macros that provide haskell style monadic
 * "do" syntax without using closures. obviously not all
 * monads can be supported; currently only the
 * std::result::Result struct works. the option struct
 * should be supportable as well.
 *
 * inspired by Guillaume Pinot's rust-mdo
 *
 *
 * Syntax:
 * `(instr)* ; ret expr`
 *
 * instr can be:
 *
 * * `pattern <- expression`: bind expression to pattern.
 *
 * * `let pattern = expression`: assign expression to pattern, as
 *   normal rust let.
 *
 * * `ign expression`: equivalent to `_ <- expression`
 *
 */

#[macro_export]
macro_rules! result_do(
  (let $p:path = $e:expr ; $( $t:tt )*) => (
    { let $p = $e ; result_do! { $( $t )* } }
  );

  ($p:pat <- $e:expr ; $( $t:tt )*) => (
    match $e {
      Ok($p)   => result_do! { $( $t )* },
      Err(err)   => Err(err)
    }
  );

  (ign $e:expr ; $( $t:tt )*) => (
    match $e {
      _ => result_do! { $( $t )* }
    }
  );

  (ret $f:expr) => (
    Ok($f)
  )
)
