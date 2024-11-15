// Allocates "Domains" in which they will point into unique addresses.
#[macro_export]
macro_rules! interrupt_declare_domains {
  ($($domain:ident),*) => {
    pub mod domains {
      struct __Bases {
        $( $domain: crate::interrupt::IrqDomainBase, )*
      }

      #[allow(non_snake_case)]
      static __BASES: __Bases = __Bases {
        $($domain: crate::interrupt::IrqDomainBase(0),)*
      };

      $(
        pub static $domain: crate::interrupt::IrqDomain =
          crate::interrupt::IrqDomain(
            &__BASES.$domain
              as *const crate::interrupt::IrqDomainBase
              as *const ());
      )*
    }
  };
}
