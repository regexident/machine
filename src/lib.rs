#![feature(log_syntax,trace_macros)]

#[macro_export]
macro_rules! machine (
  ( $machine:ident($state:ty) {
    {
      initial: $initial:path,
      error:   $error:path
    }

    $(
    event $ev:ident {
      $($tokens:tt)*
    }
    )*
  }) => (
    #[derive(PartialEq,Eq,Debug)]
    struct $machine {
      state: $state
    }

    impl $machine {
      fn new() -> $machine {
        $machine { state: $initial }
      }

      $(transitions!(
          $ev,
          $error,
          $($tokens)*
        );
      )*
    }
  )
);

macro_rules! transitions (
  ($ev:ident, $err:path,  $($state:pat => $res:expr),*) => (
    fn $ev(&mut self) -> Option<()> {
      match self.state {
        $($state => {self.state = $res; Some(())},)*
        _        => {self.state = $err; None}
      }
    }
  );
);

#[cfg(test)]
mod tests {
  #![feature(trace_macros)]
  use super::*;

  #[derive(PartialEq,Eq,Debug)]
  pub enum State {
    A, B(u8), C(u8), Error
  }

  trace_macros!(true);
  machine!(Machine(State) {
    {
      initial: State::A,
      error  : State::Error
    }

    event tr {
      State::A    => State::B(0),
      State::B(i) => State::C(i+1)
    }

    event tr2 {
      State::C(_) => State::A,
      State::A    => State::C(42) //,
      //State::B
    }
  });
  trace_macros!(false);

  #[test]
  fn a() {
    let mut m = Machine::new();
    println!("state: {:?}", m);
    let mut res = m.tr();
    println!("state({:?}): {:?}", res, m);
    res = m.tr();
    println!("state({:?}): {:?}", res, m);
    res = m.tr();
    println!("state({:?}): {:?}", res, m);
    res = m.tr2();
    println!("state({:?}): {:?}", res, m);
    res = m.tr2();
    println!("state({:?}): {:?}", res, m);
    assert!(false);
  }
}
