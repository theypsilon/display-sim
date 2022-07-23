/* Copyright (c) 2019-2022 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

#[macro_export]
macro_rules! console {
    ( $x:ident. $a:expr ) => {
        paste::expr! { web_sys::console::[<$x _1>](&($a).into()) }
    };
    ( $x:ident. $a:expr, $b:expr ) => {
        paste::expr! { web_sys::console::[<$x _2>](&($a).into(), &($b).into()); }
    };
    ( $x:ident. $a:expr, $b:expr, $c:expr  ) => {
        paste::expr! { web_sys::console::[<$x _3>](&($a).into(), &($b).into(), &($c).into()); }
    };
    ( $x:ident. $a:expr, $b:expr, $c:expr, $d:expr ) => {
        paste::expr! { web_sys::console::[<$x _4>](&($a).into(), &($b).into(), &($c).into(), &($d).into()); }
    };
    ( $x:ident. $a:expr, $b:expr, $c:expr, $d:expr, $e:expr ) => {
        paste::expr! { web_sys::console::[<$x _5>](&($a).into(), &($b).into(), &($c).into(), &($d).into(), &($e).into()); }
    };
    ( $x:ident. $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr ) => {
        paste::expr! { web_sys::console::[<$x _6>](&($a).into(), &($b).into(), &($c).into(), &($d).into(), &($e).into(), &($f).into()); }
    };
    ( $x:ident. $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr ) => {
        paste::expr! { web_sys::console::[<$x _7>](&($a).into(), &($b).into(), &($c).into(), &($d).into(), &($e).into(), &($f).into(), &($g).into()); }
    };
}
