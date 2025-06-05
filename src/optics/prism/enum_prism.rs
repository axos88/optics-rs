use crate::mapped_prism;

/// Generates a prism (getter and setter pair) for a specific enum variant.
///
/// This macro expands to a call to `mapped_prism`, creating:
/// - a getter closure that attempts to extract the fields of a specific variant
/// - a setter closure that replaces the entire enum value with a new instance of that variant
///
/// The macro handles **tuple-like**, **struct-like**, and **unit** enum variants.
///
/// For variants with:
/// - **No fields** (unit variant), the getter returns `Option<()>`
/// - **One field**, the getter returns `Option<FieldType>` directly (not wrapped in a tuple)
/// - **Multiple fields**, the getter returns `Option<(FieldType1, FieldType2, ...)>` as a tuple
///
/// # Syntax
///
/// ```ignore
/// enum_prism!(TypeName, VariantName, variant_kind, (arg1, arg2, ...))
/// ```
///
/// - `TypeName`: The name of the enum type
/// - `VariantName`: The name of the variant to target
/// - `variant_kind`: One of `tuple`, `struct`, or `unit`
/// - `(arg1, arg2, ...)`: A list of identifiers representing the fields of the variant;
///   for `unit` variants, use `()`
///
/// # Example
///
/// ```rust
/// # use optics::enum_prism;
/// # fn mapped_prism<GET, SET, T, F>(getter: GET, setter: SET) -> (GET, SET)
/// # where GET: Fn(&T) -> Option<F>, SET: Fn(&mut T, F), { (getter, setter) }
/// #[derive(Debug, Clone)]
/// enum Message {
///     Quit,
///     Move { x: i32, y: i32 },
///     Echo(String),
/// }
///
/// // Struct-like variant with multiple fields returns tuple
/// let move_prism = enum_prism!(Message, Move, struct, (x, y));
/// let m = Message::Move { x: 10, y: 20 };
/// assert_eq!(move_prism.0(&m), Some((10, 20)));
///
/// // Tuple-like variant with single field returns field directly
/// let echo_prism = enum_prism!(Message, Echo, tuple, (msg));
/// let e = Message::Echo("Hello".into());
/// assert_eq!(echo_prism.0(&e), Some("Hello".to_string()));
///
/// // Unit variant returns ()
/// let quit_prism = enum_prism!(Message, Quit, unit, ());
/// let q = Message::Quit;
/// assert_eq!(quit_prism.0(&q), Some(()));
/// ```
///
/// # Notes
///
/// - The getter returns an `Option` of the variant’s fields with the following rules:
///   - Unit variants return `Option<()>`
///   - Single-field variants return the field type directly inside `Option`
///   - Multi-field variants return a tuple of fields inside `Option`
/// - The setter replaces the enum with a new instance of the variant.
/// - Fields are cloned in the getter; therefore, field types must implement `Clone`.
///
/// # See Also
///
/// - [`mapped_prism`] for the expected function signature this macro generates.
#[macro_export]
macro_rules! enum_prism {
    // Unit variant (no args)
    ($type:path, $variant:ident, unit, ()) => {
        crate::mapped_prism(
            |input: &$type| match input {
                &<$type>::$variant => Ok(()),
                _ => Err(()),
            },
            |input: &mut $type, ()| {
                *input = <$type>::$variant;
            },
        )
    };

    // Single field tuple-like variant
    ($type:path, $variant:ident, tuple, ($arg:ident)) => {
        $crate::mapped_prism(
            |input: &$type| match input {
                &$type::$variant(ref $arg) => Ok($arg.clone()),
                _ => Err(()),
            },
            |input: &mut $type, value| {
                *input = $type::$variant(value);
            },
        )
    };

    // Multiple fields tuple-like variant
    ($type:path, $variant:ident, tuple, ($first:ident, $($rest:ident),+)) => {
        $crate::mapped_prism(
            |input: &$type| match input {
                <$type>::$variant(ref $first, $(ref $rest),+) => Ok(($first.clone(), $($rest.clone()),+)),
                _ => Err(()),
            },
            |input: &mut $type, ($first, $($rest),+)| {
                *input = <$type>::$variant($first, $($rest),+);
            },
        )
    };

    // Single field struct-like variant
    ($type:path, $variant:ident, struct, ($arg:ident)) => {
        $crate::mapped_prism(
            |input: &$type| match input {
                <$type>::$variant { ref $arg } => Ok($arg.clone()),
                _ => Err(()),
            },
            |input: &mut $type, value| {
                *input = <$type>::$variant { $arg: value };
            },
        )
    };

    // Multiple fields struct-like variant
    ($type:path, $variant:ident, struct, ($first:ident, $($rest:ident),+)) => {
        $crate::mapped_prism(
            |input: &$type| match input {
                <$type>::$variant { ref $first, $(ref $rest),+ } => Ok(($first.clone(), $($rest.clone()),+)),
                _ => Err(()),
            },
            |input: &mut $type, ($first, $($rest),+)| {
                *input = <$type>::$variant { $first, $($rest),+ };
            },
        )
    };
}
