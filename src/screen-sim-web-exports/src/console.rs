#[macro_export]
macro_rules! console {
    ( $x:ident. $a:expr ) => {
        paste::expr! { web_sys::console::[<$x _1>](&$a.into()); }
    };
    ( $x:ident. $a:expr, $b:expr ) => {
        paste::expr! { web_sys::console::[<$x _2>](&$a.into(), &$b.into()); }
    };
    ( $x:ident. $a:expr, $b:expr, $c:expr  ) => {
        paste::expr! { web_sys::console::[<$x _3>](&$a.into(), &$b.into(), &$c.into()); }
    };
    ( $x:ident. $a:expr, $b:expr, $c:expr, $d:expr ) => {
        paste::expr! { web_sys::console::[<$x _4>](&$a.into(), &$b.into(), &$c.into(), &$d.into()); }
    };
    ( $x:ident. $a:expr, $b:expr, $c:expr, $d:expr, $e:expr ) => {
        paste::expr! { web_sys::console::[<$x _5>](&$a.into(), &$b.into(), &$c.into(), &$d.into(), &$e.into()); }
    };
    ( $x:ident. $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr ) => {
        paste::expr! { web_sys::console::[<$x _6>](&$a.into(), &$b.into(), &$c.into(), &$d.into(), &$e.into(), &$f.into()); }
    };
    ( $x:ident. $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr ) => {
        paste::expr! { web_sys::console::[<$x _7>](&$a.into(), &$b.into(), &$c.into(), &$d.into(), &$e.into(), &$f.into(), &$g.into()); }
    };
}
