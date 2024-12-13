macro_rules! bulk_impl {
    ($other:ty, $specialized:expr) => {
        /// Like from_color, but on larger quantities to make use of SIMD batching
        fn from_color_bulk(output: &mut [Self], input: &[$other])
        where
            Self: Sized,
        {
            // #[cfg(feature = "simd")]
            $specialized(output, input);
            // #[cfg!(not(feature = "simd"))]

            for (i, val) in input.iter().enumerate() {
                Self::from_color(&mut output[i], val);
            }
        }
    };
    ($other:ty) => {
        /// Like from_color, but on larger quantities to make use of SIMD batching
        fn from_color_bulk(output: &mut [Self], input: &[$other])
        where
            Self: Sized,
        {
            for (i, val) in input.iter().enumerate() {
                Self::from_color(&mut output[i], val);
            }
        }
    };
}
// This needs to come after the macro definition as per exporting rules for decl. macros
use crate::Primitive;
pub(crate) use bulk_impl;
use crate::color::FromPrimitive;

pub(crate) fn same_sized_impl<Other, Slf, OtherPrim: Primitive, SlfPrim: Primitive>(
	output: &mut [Slf],
	input: &[Other],
) where
	Other: Sized,
{
    assert_eq!(
        input.len(),
        output.len(),
        "Input and output slices must have the same length."
    );
    // Safe because `Foo` is `#[repr(transparent)]`
    let input: &[OtherPrim] =
        unsafe { std::slice::from_raw_parts(input.as_ptr() as *const OtherPrim, input.len()) };

    // Same goes for T
    let output: &mut [SlfPrim] =
        unsafe { std::slice::from_raw_parts_mut(output.as_ptr() as *mut SlfPrim, output.len()) };

    SlfPrim::from_bulk_primitive(input, output);
}
