//! cmp
//!

/// impl_ops_cmp without Copy derive
#[macro_export]
macro_rules! impl_ops_cmp {
  (impl $imp:ident, $imo:ident, $method:ident for $t:ty, $u:ty, $sgn:expr) => {
/*
    /// impl $imp&lt;&amp;$u&gt; for &amp;mut $t
    /// - expect $imp as PartialEq or Eq
    impl<'a, 'b> $imp<&'b $u> for &'a mut $t {
      /// eq &amp;mut $t == &amp;$u
      #[inline]
      fn eq(&self, rhs: &&'b $u) -> bool {
        self.$method(*rhs) == 0
      }
    }

    /// impl $imo&lt;&amp;$u&gt; for &amp;mut $t
    /// - expect $imo as PartialOrd
    impl<'a, 'b> $imo<&'b $u> for &'a mut $t {
      /// partial_cmp &amp;mut $t - &amp;$u
      #[inline]
      fn partial_cmp(&self, rhs: &&'b $u) -> Option<Ordering> {
        (self.$method(*rhs) * $sgn).partial_cmp(&0)
//        let c = self.$method(*rhs) * $sgn;
//        if c == 0 { Some(Ordering::Equal) }
//        else if c < 0 { Some(Ordering::Less) }
//        else if c > 0 { Some(Ordering::Greater) }
//        else { None }
      }
    }
*/
    /// impl $imp&lt;$u&gt; for &amp;mut $t
    /// - expect $imp as PartialEq or Eq
    impl<'a> $imp<$u> for &'a mut $t {
      /// eq &amp;mut $t == $u
      #[inline]
      fn eq(&self, rhs: &$u) -> bool {
        self.$method(rhs) == 0
      }
    }

    /// impl $imo&lt;$u&gt; for &amp;mut $t
    /// - expect $imo as PartialOrd
    impl<'a> $imo<$u> for &'a mut $t {
      /// partial_cmp &amp;mut $t - $u
      #[inline]
      fn partial_cmp(&self, rhs: &$u) -> Option<Ordering> {
        (self.$method(rhs) * $sgn).partial_cmp(&0)
      }
    }

    /// impl $imp&lt;$u&gt; for $t
    /// - expect $imp as PartialEq or Eq
    impl $imp<$u> for $t {
      /// eq $t == $u
      #[inline]
      fn eq(&self, rhs: &$u) -> bool {
        self.$method(rhs) == 0
      }
    }

    /// impl $imo&lt;$u&gt; for $t
    /// - expect $imo as PartialOrd
    impl $imo<$u> for $t {
      /// partial_cmp $t - $u
      #[inline]
      fn partial_cmp(&self, rhs: &$u) -> Option<Ordering> {
        (self.$method(rhs) * $sgn).partial_cmp(&0)
      }
    }
  };
}
pub use impl_ops_cmp;

/// impl_ops_cmp_p without Copy derive
#[macro_export]
macro_rules! impl_ops_cmp_p {
  (impl $imp:ident, $imo:ident, $method:ident for $t:ty, $u:ty, $sgn:expr) => {
    /// impl $imp&lt;$u&gt; for &amp;mut $t
    /// - expect $imp as PartialEq or Eq
    impl<'a> $imp<$u> for &'a mut $t {
      /// eq &amp;mut $t == $u
      #[inline]
      fn eq(&self, rhs: &$u) -> bool {
        // not use self.eq to avoid recursion infinite
        <$t>::eq(self, rhs)
      }
    }

    /// impl $imo&lt;$u&gt; for &amp;mut $t
    /// - expect $imo as PartialOrd
    impl<'a> $imo<$u> for &'a mut $t {
      /// partial_cmp &amp;mut $t - $u
      #[inline]
      fn partial_cmp(&self, rhs: &$u) -> Option<Ordering> {
        // not use self.partial_cmp to avoid recursion infinite
        <$t>::partial_cmp(self, rhs)
      }
    }

    /// impl $imp&lt;$u&gt; for &amp;$t
    /// - expect $imp as PartialEq or Eq
    impl<'a> $imp<$u> for &'a $t {
      /// eq &amp;$t == $u
      #[inline]
      fn eq(&self, rhs: &$u) -> bool {
        // not use self.eq to avoid recursion infinite
        <$t>::eq(self, rhs)
      }
    }

    /// impl $imo&lt;$u&gt; for &amp;$t
    /// - expect $imo as PartialOrd
    impl<'a> $imo<$u> for &'a $t {
      /// partial_cmp &amp;$t - $u
      #[inline]
      fn partial_cmp(&self, rhs: &$u) -> Option<Ordering> {
        // not use self.partial_cmp to avoid recursion infinite
        <$t>::partial_cmp(self, rhs)
      }
    }

    /// impl $imp&lt;$u&gt; for $t
    /// - expect $imp as PartialEq or Eq
    impl $imp<$u> for $t {
      /// eq $t == $u
      #[inline]
      fn eq(&self, rhs: &$u) -> bool {
        self.$method(*rhs) == 0
      }
    }

    /// impl $imo&lt;$u&gt; for $t
    /// - expect $imo as PartialOrd
    impl $imo<$u> for $t {
      /// partial_cmp $t - $u
      #[inline]
      fn partial_cmp(&self, rhs: &$u) -> Option<Ordering> {
        (self.$method(*rhs) * $sgn).partial_cmp(&0)
      }
    }

    impl_ops_cmp_q!{impl $imp, $imo, for $u, $t}
  };
}
pub use impl_ops_cmp_p;

/// impl_ops_cmp_q without Copy derive
#[macro_export]
macro_rules! impl_ops_cmp_q {
  (impl $imp:ident, $imo:ident, for $t:ty, $u:ty) => {
    /// impl $imp&lt;&amp;mut $u&gt; for $t
    /// - expect $imp as PartialEq or Eq
    impl<'a> $imp<&'a mut $u> for $t {
      /// eq $t == &amp;mut $u
      #[inline]
      fn eq(&self, rhs: &&'a mut $u) -> bool {
        (*rhs).eq(self)
      }
    }

    /// impl $imo&lt;&amp;mut $u&gt; for $t
    /// - expect $imo as PartialOrd
    impl<'a> $imo<&'a mut $u> for $t {
      /// partial_cmp $t - &amp;mut $u
      #[inline]
      fn partial_cmp(&self, rhs: &&'a mut $u) -> Option<Ordering> {
        (*rhs).partial_cmp(self).map(Ordering::reverse)
      }
    }

    /// impl $imp&lt;&amp;$u&gt; for $t
    /// - expect $imp as PartialEq or Eq
    impl<'a> $imp<&'a $u> for $t {
      /// eq $t == &amp;$u
      #[inline]
      fn eq(&self, rhs: &&'a $u) -> bool {
        (*rhs).eq(self)
      }
    }

    /// impl $imo&lt;&amp;$u&gt; for $t
    /// - expect $imo as PartialOrd
    impl<'a> $imo<&'a $u> for $t {
      /// partial_cmp $t - &amp;$u
      #[inline]
      fn partial_cmp(&self, rhs: &&'a $u) -> Option<Ordering> {
        (*rhs).partial_cmp(self).map(Ordering::reverse)
      }
    }

    /// impl $imp&lt;$u&gt; for $t
    /// - expect $imp as PartialEq or Eq
    impl $imp<$u> for $t {
      /// eq $t == $u
      #[inline]
      fn eq(&self, rhs: &$u) -> bool {
        rhs.eq(self)
      }
    }

    /// impl $imo&lt;$u&gt; for $t
    /// - expect $imo as PartialOrd
    impl $imo<$u> for $t {
      /// partial_cmp $t - $u
      #[inline]
      fn partial_cmp(&self, rhs: &$u) -> Option<Ordering> {
        rhs.partial_cmp(self).map(Ordering::reverse)
      }
    }
  };
}
pub use impl_ops_cmp_q;
