/// Create a ['RustArray'] containing the arguments.
///
/// `rarray!` allows `RustArray`s to be defined with the same syntax as array expressions.
/// There are two forms of this macro:
///
/// - Create a [`RustArray`] containing a given list of elements:
///
/// ```
/// let a = rarray![1, 2, 3];
/// assert_eq!(a[0], 1);
/// assert_eq!(a[1], 2);
/// assert_eq!(a[2], 3);
/// ```
///
/// - Create a [`RustArray`] from a given element and size:
///
/// ```
/// let a = rarray![1; 3];
/// assert_eq!(a.data, [1, 1, 1]);
/// ```
///
/// Note that unlike array expressions this syntax supports all elements
/// which implement [`Clone`] and the number of elements doesn't have to be
/// a constant.
///
/// This will use `clone` to duplicate an expression, so one should be careful
/// using this with types having a nonstandard `Clone` implementation. For
/// example, `rarray![Rc::new(1); 5]` will create a vector of five references
/// to the same boxed integer value, not five references pointing to independently
/// boxed integers.
///
/// Also, note that `rarray![expr; 0]` is allowed, and produces an empty vector.
/// This will still evaluate `expr`, however, and immediately drop the resulting value, so
/// be mindful of side effects.
///
/// [`RustArray`]: crate::num_core::RustArray
/// Description based on vec's macro from the rust std source
#[cfg(not(no_global_oom_handling))]
#[macro_export]
#[stable(feature = "rust1", since = "1.90.0")]
#[rustc_diagnostic_item = "array_macro"]
#[allow_internal_unstable(rustc_attrs, liballoc_internals)]
macro_rules! rarray {
    () => (
        $crate::num_core::RustArray::new()
    );
    ($elem:expr; $n:expr) => (
        //$crate::num_core::from_elem($elem, $n)
        ///--À finir à partir d'ici--
    );
    ($($x:expr),+ $(,)?) => (
        <[_]>::into_vec(
            // Using the intrinsic produces a dramatic improvement in stack usage for
            // unoptimized programs using this code path to construct large Vecs.
            $crate::boxed::box_new([$($x),+])
        )
    );
}