#[doc(hidden)]
macro_rules! _parse_unary_op {
	(-, $($t:tt)+) => (_impl_unary_op_internal!(Neg, neg, $($t)+););
	(!, $($t:tt)+) => (_impl_unary_op_internal!(Not, not, $($t)+););
}

#[doc(hidden)]
macro_rules! _impl_unary_op_internal {
	($ops_trait:ident, $ops_fn:ident, &$lhs:ty, $out:ty, $lhs_i:ident, $(#[$attrs:meta])* $body:block $($generic_params:tt)*) => {
		impl$($generic_params)* ::core::ops::$ops_trait for &$lhs {
			type Output = $out;

			$(#[$attrs])*
			fn $ops_fn(self) -> Self::Output {
				let $lhs_i = self;
				$body
			}
		}
	};
	($ops_trait:ident, $ops_fn:ident, $lhs:ty, $out:ty, $lhs_i:ident, $(#[$attrs:meta])* $body:block $($generic_params:tt)*) => {
		impl$($generic_params)* ::core::ops::$ops_trait for $lhs {
			type Output = $out;

			$(#[$attrs])*
			fn $ops_fn(self) -> Self::Output {
				let $lhs_i = self;
				$body
			}
		}
	};
}
