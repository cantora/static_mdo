#![crate_name = "static_mdo"]
#![crate_type = "rlib"]
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

#[macro_export]
macro_rules! result_tag_do(
  (let $p:path = $e:expr ; $( $t:tt )*) => (
    { let $p = $e ; result_tag_do! { $( $t )* } }
  );

  ($tag:expr : $p:pat <- $e:expr ; $( $t:tt )*) => (
    match $e {
      Ok($p)   => result_tag_do! { $( $t )* },
      Err(err)   => Err(($tag, err))
    }
  );

  (ign $e:expr ; $( $t:tt )*) => (
    match $e {
      _ => result_tag_do! { $( $t )* }
    }
  );

  (ret $f:expr) => (
    Ok($f)
  )
)

#[macro_export]
macro_rules! result_for(
  ($p:pat in $e:expr $bl:block) => ({
    let mut itr_done = false;
    let mut status = match ($e).next() {
      Some(x) => match x {
        Ok($p)   => { $bl; None },
        Err(err) => Some(err) 
      },
      None    => {
        itr_done = true;
        None 
      }
    };

    if !itr_done {
      for x in $e {
        match x {
          Ok($p)   => { $bl; },
          Err(err) => {
            status = Some(err);
            break;
          }
        }
      }
    }

    status
  });
)

/**********************
 * repeatedly evaluate an expression that
 * produces a result-like type until that
 * expression produces an Err(...), then
 * return that error.
 */
#[macro_export]
macro_rules! result_repeat(
  ( $p:pat <- $e:expr => $bl:block) => ({
    let mut status = $e;

    loop {
      match status {
        Ok($p) => { $bl;   }
        _      => { break; }
      }

      status = $e;
    }

    status
  });

  ( $e:expr => $bl:block) => ({
    let mut status = $e;

    loop {
      if status.is_ok() {
        $bl;
      }
      else {
        break;
      }

      status = $e;
    }

    status
  });
)

#[macro_export]
macro_rules! result_unwrap_or_return(
  ( $e:expr => $bl:block) => ({
    match $e {
      Err(_)       => {
        return $bl;
      }
      Ok(unwrapped) => unwrapped
    }
  });
  ( $p:pat <- $e:expr => $bl:block) => ({
    match $e {
      Err($p)       => {
        return $bl;
      }
      Ok(unwrapped) => unwrapped
    }
  });
)

#[macro_export]
macro_rules! result_on_err(
  ($p:pat <- $e:expr => $bl:block) => ({
    match $e {
      Err($p) => { $bl; }
      _       => {      }
    }
  });
)

/* this kinda works, but its wonky because
 * matching the list of statements limits normal
 * syntax like match x { ... }. seems like i want
 * be able to match the "inside" of a block, but
 * im not sure how to do that.
 * macro_rules! result_for2(
 *   ( [ $p:pat <- $e:expr ] $( $st:stmt );* ; ) => ({
 *     let mut itr_done = false;
 *     let mut status = match ($e).next() {
 *       Some(x) => match x {
 *         Ok($p)   => { { $( $st );* }; None },
 *         Err(err) => Some(err) 
 *       },
 *       None    => {
 *         itr_done = true;
 *         None 
 *       }
 *     };
 * 
 *     if !itr_done {
 *       for x in $e {
 *         match x {
 *           Ok($p)   => { { $( $st );* }; },
 *           Err(err) => {
 *             status = Some(err);
 *             break;
 *           }
 *         }
 *       }
 *     }
 * 
 *     status
 *   });
 * )
 */
