// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::cast::*;

const MIN_DECIMAL_F64_POWER: i32 = -(Decimal256Type::MAX_SCALE as i32);
// Bit patterns produced by `10_f64.powi(exp)` for every valid Arrow decimal
// scale. Using the exact bit patterns preserves the existing lossy cast results,
// including the rounding of powers that cannot be represented exactly as f64.
const DECIMAL_F64_POWERS_OF_TEN: [f64; Decimal256Type::MAX_SCALE as usize * 2 + 1] = [
    f64::from_bits(0x3027288e1271f512), // 10^-76
    f64::from_bits(0x305cf2b1970e7257), // 10^-75
    f64::from_bits(0x309217aefe690776), // 10^-74
    f64::from_bits(0x30c69d9abe034954), // 10^-73
    f64::from_bits(0x30fc45016d841ba9), // 10^-72
    f64::from_bits(0x3131ab20e472914a), // 10^-71
    f64::from_bits(0x316615e91d8f359c), // 10^-70
    f64::from_bits(0x319b9b6364f30304), // 10^-69
    f64::from_bits(0x31d1411e1f17e1e2), // 10^-68
    f64::from_bits(0x32059165a6ddda5b), // 10^-67
    f64::from_bits(0x323af5bf109550f1), // 10^-66
    f64::from_bits(0x3270d9976a5d5296), // 10^-65
    f64::from_bits(0x32a50ffd44f4a73c), // 10^-64
    f64::from_bits(0x32da53fc9631d10c), // 10^-63
    f64::from_bits(0x3310747ddddf22a8), // 10^-62
    f64::from_bits(0x3344919d5556eb52), // 10^-61
    f64::from_bits(0x3379b604aaaca627), // 10^-60
    f64::from_bits(0x33b011c2eaabe7d8), // 10^-59
    f64::from_bits(0x33e41633a556e1cd), // 10^-58
    f64::from_bits(0x34191bc08eac9a40), // 10^-57
    f64::from_bits(0x344f62b0b257c0d1), // 10^-56
    f64::from_bits(0x34839dae6f76d883), // 10^-55
    f64::from_bits(0x34b8851a0b548ea3), // 10^-54
    f64::from_bits(0x34eea6608e29b24d), // 10^-53
    f64::from_bits(0x352327fc58da0f70), // 10^-52
    f64::from_bits(0x3557f1fb6f10934c), // 10^-51
    f64::from_bits(0x358dee7a4ad4b81e), // 10^-50
    f64::from_bits(0x35c2b50c6ec4f313), // 10^-49
    f64::from_bits(0x35f7624f8a762fd8), // 10^-48
    f64::from_bits(0x362d3ae36d13bbce), // 10^-47
    f64::from_bits(0x366244ce242c5561), // 10^-46
    f64::from_bits(0x3696d601ad376ab9), // 10^-45
    f64::from_bits(0x36cc8b8218854567), // 10^-44
    f64::from_bits(0x3701d7314f534b61), // 10^-43
    f64::from_bits(0x37364cfda3281e38), // 10^-42
    f64::from_bits(0x376be03d0bf225c7), // 10^-41
    f64::from_bits(0x37a16c262777579c), // 10^-40
    f64::from_bits(0x37d5c72fb1552d83), // 10^-39
    f64::from_bits(0x380b38fb9daa78e4), // 10^-38
    f64::from_bits(0x3841039d428a8b8e), // 10^-37
    f64::from_bits(0x38754484932d2e72), // 10^-36
    f64::from_bits(0x38aa95a5b7f87a0f), // 10^-35
    f64::from_bits(0x38e09d8792fb4c49), // 10^-34
    f64::from_bits(0x3914c4e977ba1f5b), // 10^-33
    f64::from_bits(0x3949f623d5a8a732), // 10^-32
    f64::from_bits(0x398039d665896880), // 10^-31
    f64::from_bits(0x39b4484bfeebc29f), // 10^-30
    f64::from_bits(0x39e95a5efea6b348), // 10^-29
    f64::from_bits(0x3a1fb0f6be50601a), // 10^-28
    f64::from_bits(0x3a53ce9a36f23c10), // 10^-27
    f64::from_bits(0x3a88c240c4aecb13), // 10^-26
    f64::from_bits(0x3abef2d0f5da7dd8), // 10^-25
    f64::from_bits(0x3af357c299a88ea8), // 10^-24
    f64::from_bits(0x3b282db34012b252), // 10^-23
    f64::from_bits(0x3b5e392010175ee6), // 10^-22
    f64::from_bits(0x3b92e3b40a0e9b4f), // 10^-21
    f64::from_bits(0x3bc79ca10c924223), // 10^-20
    f64::from_bits(0x3bfd83c94fb6d2ac), // 10^-19
    f64::from_bits(0x3c32725dd1d243ac), // 10^-18
    f64::from_bits(0x3c670ef54646d497), // 10^-17
    f64::from_bits(0x3c9cd2b297d889bc), // 10^-16
    f64::from_bits(0x3cd203af9ee75616), // 10^-15
    f64::from_bits(0x3d06849b86a12b9b), // 10^-14
    f64::from_bits(0x3d3c25c268497682), // 10^-13
    f64::from_bits(0x3d719799812dea11), // 10^-12
    f64::from_bits(0x3da5fd7fe1796495), // 10^-11
    f64::from_bits(0x3ddb7cdfd9d7bdbb), // 10^-10
    f64::from_bits(0x3e112e0be826d695), // 10^-9
    f64::from_bits(0x3e45798ee2308c3a), // 10^-8
    f64::from_bits(0x3e7ad7f29abcaf48), // 10^-7
    f64::from_bits(0x3eb0c6f7a0b5ed8d), // 10^-6
    f64::from_bits(0x3ee4f8b588e368f1), // 10^-5
    f64::from_bits(0x3f1a36e2eb1c432d), // 10^-4
    f64::from_bits(0x3f50624dd2f1a9fc), // 10^-3
    f64::from_bits(0x3f847ae147ae147b), // 10^-2
    f64::from_bits(0x3fb999999999999a), // 10^-1
    f64::from_bits(0x3ff0000000000000), // 10^0
    f64::from_bits(0x4024000000000000), // 10^1
    f64::from_bits(0x4059000000000000), // 10^2
    f64::from_bits(0x408f400000000000), // 10^3
    f64::from_bits(0x40c3880000000000), // 10^4
    f64::from_bits(0x40f86a0000000000), // 10^5
    f64::from_bits(0x412e848000000000), // 10^6
    f64::from_bits(0x416312d000000000), // 10^7
    f64::from_bits(0x4197d78400000000), // 10^8
    f64::from_bits(0x41cdcd6500000000), // 10^9
    f64::from_bits(0x4202a05f20000000), // 10^10
    f64::from_bits(0x42374876e8000000), // 10^11
    f64::from_bits(0x426d1a94a2000000), // 10^12
    f64::from_bits(0x42a2309ce5400000), // 10^13
    f64::from_bits(0x42d6bcc41e900000), // 10^14
    f64::from_bits(0x430c6bf526340000), // 10^15
    f64::from_bits(0x4341c37937e08000), // 10^16
    f64::from_bits(0x4376345785d8a000), // 10^17
    f64::from_bits(0x43abc16d674ec800), // 10^18
    f64::from_bits(0x43e158e460913d00), // 10^19
    f64::from_bits(0x4415af1d78b58c40), // 10^20
    f64::from_bits(0x444b1ae4d6e2ef50), // 10^21
    f64::from_bits(0x4480f0cf064dd592), // 10^22
    f64::from_bits(0x44b52d02c7e14af6), // 10^23
    f64::from_bits(0x44ea784379d99db4), // 10^24
    f64::from_bits(0x45208b2a2c280291), // 10^25
    f64::from_bits(0x4554adf4b7320335), // 10^26
    f64::from_bits(0x4589d971e4fe8402), // 10^27
    f64::from_bits(0x45c027e72f1f1281), // 10^28
    f64::from_bits(0x45f431e0fae6d721), // 10^29
    f64::from_bits(0x46293e5939a08cea), // 10^30
    f64::from_bits(0x465f8def8808b024), // 10^31
    f64::from_bits(0x4693b8b5b5056e17), // 10^32
    f64::from_bits(0x46c8a6e32246c99d), // 10^33
    f64::from_bits(0x46fed09bead87c04), // 10^34
    f64::from_bits(0x4733426172c74d82), // 10^35
    f64::from_bits(0x476812f9cf7920e3), // 10^36
    f64::from_bits(0x479e17b84357691c), // 10^37
    f64::from_bits(0x47d2ced32a16a1b1), // 10^38
    f64::from_bits(0x48078287f49c4a1e), // 10^39
    f64::from_bits(0x483d6329f1c35ca5), // 10^40
    f64::from_bits(0x48725dfa371a19e7), // 10^41
    f64::from_bits(0x48a6f578c4e0a061), // 10^42
    f64::from_bits(0x48dcb2d6f618c879), // 10^43
    f64::from_bits(0x4911efc659cf7d4c), // 10^44
    f64::from_bits(0x49466bb7f0435c9f), // 10^45
    f64::from_bits(0x497c06a5ec5433c6), // 10^46
    f64::from_bits(0x49b18427b3b4a05c), // 10^47
    f64::from_bits(0x49e5e531a0a1c873), // 10^48
    f64::from_bits(0x4a1b5e7e08ca3a90), // 10^49
    f64::from_bits(0x4a511b0ec57e649a), // 10^50
    f64::from_bits(0x4a8561d276ddfdc0), // 10^51
    f64::from_bits(0x4ababa4714957d30), // 10^52
    f64::from_bits(0x4af0b46c6cdd6e3e), // 10^53
    f64::from_bits(0x4b24e1878814c9ce), // 10^54
    f64::from_bits(0x4b5a19e96a19fc41), // 10^55
    f64::from_bits(0x4b905031e2503da9), // 10^56
    f64::from_bits(0x4bc4643e5ae44d14), // 10^57
    f64::from_bits(0x4bf97d4df19d6058), // 10^58
    f64::from_bits(0x4c2fdca16e04b86e), // 10^59
    f64::from_bits(0x4c63e9e4e4c2f344), // 10^60
    f64::from_bits(0x4c98e45e1df3b015), // 10^61
    f64::from_bits(0x4ccf1d75a5709c1b), // 10^62
    f64::from_bits(0x4d03726987666191), // 10^63
    f64::from_bits(0x4d384f03e93ff9f6), // 10^64
    f64::from_bits(0x4d6e62c4e38ff874), // 10^65
    f64::from_bits(0x4da2fdbb0e39fb48), // 10^66
    f64::from_bits(0x4dd7bd29d1c87a1a), // 10^67
    f64::from_bits(0x4e0dac74463a98a1), // 10^68
    f64::from_bits(0x4e428bc8abe49f64), // 10^69
    f64::from_bits(0x4e772ebad6ddc73e), // 10^70
    f64::from_bits(0x4eacfa698c95390d), // 10^71
    f64::from_bits(0x4ee21c81f7dd43a8), // 10^72
    f64::from_bits(0x4f16a3a275d49492), // 10^73
    f64::from_bits(0x4f4c4c8b1349b9b7), // 10^74
    f64::from_bits(0x4f81afd6ec0e1412), // 10^75
    f64::from_bits(0x4fb61bcca7119917), // 10^76
];

/// Returns `10^exp` from a precomputed lookup table for decimal scales. Scales
/// outside the range supported by Arrow decimal types retain `f64::powi`
/// semantics rather than indexing outside the table.
#[inline]
pub(super) fn f64_power_of_ten(exp: i32) -> f64 {
    let Some(index) = exp
        .checked_sub(MIN_DECIMAL_F64_POWER)
        .and_then(|index| usize::try_from(index).ok())
    else {
        return 10_f64.powi(exp);
    };

    DECIMAL_F64_POWERS_OF_TEN
        .get(index)
        .copied()
        .unwrap_or_else(|| 10_f64.powi(exp))
}

/// A utility trait that provides checked conversions between
/// decimal types inspired by [`NumCast`]
pub trait DecimalCast: Sized {
    /// Convert the decimal to an i32
    fn to_i32(self) -> Option<i32>;

    /// Convert the decimal to an i64
    fn to_i64(self) -> Option<i64>;

    /// Convert the decimal to an i128
    fn to_i128(self) -> Option<i128>;

    /// Convert the decimal to an i256
    fn to_i256(self) -> Option<i256>;

    /// Convert a decimal from a decimal
    fn from_decimal<T: DecimalCast>(n: T) -> Option<Self>;

    /// Convert a decimal from a f64
    fn from_f64(n: f64) -> Option<Self>;
}

impl DecimalCast for i32 {
    fn to_i32(self) -> Option<i32> {
        Some(self)
    }

    fn to_i64(self) -> Option<i64> {
        Some(self as i64)
    }

    fn to_i128(self) -> Option<i128> {
        Some(self as i128)
    }

    fn to_i256(self) -> Option<i256> {
        Some(i256::from_i128(self as i128))
    }

    fn from_decimal<T: DecimalCast>(n: T) -> Option<Self> {
        n.to_i32()
    }

    fn from_f64(n: f64) -> Option<Self> {
        n.to_i32()
    }
}

impl DecimalCast for i64 {
    fn to_i32(self) -> Option<i32> {
        i32::try_from(self).ok()
    }

    fn to_i64(self) -> Option<i64> {
        Some(self)
    }

    fn to_i128(self) -> Option<i128> {
        Some(self as i128)
    }

    fn to_i256(self) -> Option<i256> {
        Some(i256::from_i128(self as i128))
    }

    fn from_decimal<T: DecimalCast>(n: T) -> Option<Self> {
        n.to_i64()
    }

    fn from_f64(n: f64) -> Option<Self> {
        // Call implementation explicitly otherwise this resolves to `to_i64`
        // in arrow-buffer that behaves differently.
        num_traits::ToPrimitive::to_i64(&n)
    }
}

impl DecimalCast for i128 {
    fn to_i32(self) -> Option<i32> {
        i32::try_from(self).ok()
    }

    fn to_i64(self) -> Option<i64> {
        i64::try_from(self).ok()
    }

    fn to_i128(self) -> Option<i128> {
        Some(self)
    }

    fn to_i256(self) -> Option<i256> {
        Some(i256::from_i128(self))
    }

    fn from_decimal<T: DecimalCast>(n: T) -> Option<Self> {
        n.to_i128()
    }

    fn from_f64(n: f64) -> Option<Self> {
        n.to_i128()
    }
}

impl DecimalCast for i256 {
    fn to_i32(self) -> Option<i32> {
        self.to_i128().map(|x| i32::try_from(x).ok())?
    }

    fn to_i64(self) -> Option<i64> {
        self.to_i128().map(|x| i64::try_from(x).ok())?
    }

    fn to_i128(self) -> Option<i128> {
        self.to_i128()
    }

    fn to_i256(self) -> Option<i256> {
        Some(self)
    }

    fn from_decimal<T: DecimalCast>(n: T) -> Option<Self> {
        n.to_i256()
    }

    fn from_f64(n: f64) -> Option<Self> {
        i256::from_f64(n)
    }
}

/// Construct closures to upscale decimals from `(input_precision, input_scale)` to
/// `(output_precision, output_scale)`.
///
/// Returns `(f_fallible, f_infallible)` where:
/// * `f_fallible` yields `None` when the requested cast would overflow
/// * `f_infallible` is present only when every input is guaranteed to succeed; otherwise it is `None`
///   and callers must fall back to `f_fallible`
///
/// Returns `None` if the required scale increase `delta_scale = output_scale - input_scale`
/// exceeds the supported precomputed precision table `O::MAX_FOR_EACH_PRECISION`.
/// In that case, the caller should treat this as an overflow for the output scale
/// and handle it accordingly (e.g., return a cast error).
#[allow(clippy::type_complexity)]
fn make_upscaler<I: DecimalType, O: DecimalType>(
    input_precision: u8,
    input_scale: i8,
    output_precision: u8,
    output_scale: i8,
) -> Option<(
    impl Fn(I::Native) -> Option<O::Native>,
    Option<impl Fn(I::Native) -> O::Native>,
)>
where
    I::Native: DecimalCast + ArrowNativeTypeOp,
    O::Native: DecimalCast + ArrowNativeTypeOp,
{
    let delta_scale = output_scale - input_scale;

    let mul = O::power_of_ten(delta_scale as u32)?;
    let f_fallible = move |x| O::Native::from_decimal(x).and_then(|x| x.mul_checked(mul).ok());

    // if the gain in precision (digits) is greater than the multiplication due to scaling
    // every number will fit into the output type
    // Example: If we are starting with any number of precision 5 [xxxxx],
    // then an increase of scale by 3 will have the following effect on the representation:
    // [xxxxx] -> [xxxxx000], so for the cast to be infallible, the output type
    // needs to provide at least 8 digits precision
    let is_infallible_cast = (input_precision as i8) + delta_scale <= (output_precision as i8);
    let f_infallible = is_infallible_cast
        .then_some(move |x| O::Native::from_decimal(x).unwrap().mul_wrapping(mul));
    Some((f_fallible, f_infallible))
}

/// Construct closures to downscale decimals from `(input_precision, input_scale)` to
/// `(output_precision, output_scale)`.
///
/// Returns `(f_fallible, f_infallible)` where:
/// * `f_fallible` yields `None` when the requested cast would overflow
/// * `f_infallible` is present only when every input is guaranteed to succeed; otherwise it is `None`
///   and callers must fall back to `f_fallible`
///
/// Returns `None` if the required scale reduction `delta_scale = input_scale - output_scale`
/// exceeds the supported precomputed precision table `I::MAX_FOR_EACH_PRECISION`.
/// In this scenario, any value would round to zero (e.g., dividing by 10^k where k exceeds the
/// available precision). Callers should therefore produce zero values (preserving nulls) rather
/// than returning an error.
#[allow(clippy::type_complexity)]
fn make_downscaler<I: DecimalType, O: DecimalType>(
    input_precision: u8,
    input_scale: i8,
    output_precision: u8,
    output_scale: i8,
) -> Option<(
    impl Fn(I::Native) -> Option<O::Native>,
    Option<impl Fn(I::Native) -> O::Native>,
)>
where
    I::Native: DecimalCast + ArrowNativeTypeOp,
    O::Native: DecimalCast + ArrowNativeTypeOp,
{
    let delta_scale = input_scale - output_scale;

    // delta_scale is guaranteed to be > 0, but may also be larger than I::MAX_PRECISION. If so, the
    // scale change divides out more digits than the input has precision and the result of the cast
    // is always zero. For example, if we try to apply delta_scale=10 a decimal32 value, the largest
    // possible result is 999999999/10000000000 = 0.0999999999, which rounds to zero. Smaller values
    // (e.g. 1/10000000000) or larger delta_scale (e.g. 999999999/10000000000000) produce even
    // smaller results, which also round to zero. In that case, just return an array of zeros.
    let div = I::power_of_ten(delta_scale as u32)?;
    let half = div.div_wrapping(I::Native::ONE.add_wrapping(I::Native::ONE));
    let half_neg = half.neg_wrapping();

    let f_fallible = move |x: I::Native| {
        // div is >= 10 and so this cannot overflow
        let d = x.div_wrapping(div);
        let r = x.mod_wrapping(div);

        // Round result
        let adjusted = match x >= I::Native::ZERO {
            true if r >= half => d.add_wrapping(I::Native::ONE),
            false if r <= half_neg => d.sub_wrapping(I::Native::ONE),
            _ => d,
        };
        O::Native::from_decimal(adjusted)
    };

    // if the reduction of the input number through scaling (dividing) is greater
    // than a possible precision loss (plus potential increase via rounding)
    // every input number will fit into the output type
    // Example: If we are starting with any number of precision 5 [xxxxx],
    // then and decrease the scale by 3 will have the following effect on the representation:
    // [xxxxx] -> [xx] (+ 1 possibly, due to rounding).
    // The rounding may add a digit, so the cast to be infallible,
    // the output type needs to have at least 3 digits of precision.
    // e.g. Decimal(5, 3) 99.999 to Decimal(3, 0) will result in 100:
    // [99999] -> [99] + 1 = [100], a cast to Decimal(2, 0) would not be possible
    let is_infallible_cast = (input_precision as i8) - delta_scale < (output_precision as i8);
    let f_infallible = is_infallible_cast.then_some(move |x| f_fallible(x).unwrap());
    Some((f_fallible, f_infallible))
}

/// Apply the rescaler function to the value.
/// If the rescaler is infallible, use the infallible function.
/// Otherwise, use the fallible function and validate the precision.
fn apply_rescaler<I: DecimalType, O: DecimalType>(
    value: I::Native,
    output_precision: u8,
    f: impl Fn(I::Native) -> Option<O::Native>,
    f_infallible: Option<impl Fn(I::Native) -> O::Native>,
) -> Option<O::Native>
where
    I::Native: DecimalCast,
    O::Native: DecimalCast,
{
    if let Some(f_infallible) = f_infallible {
        Some(f_infallible(value))
    } else {
        f(value).filter(|v| O::is_valid_decimal_precision(*v, output_precision))
    }
}

/// Rescales a decimal value from `(input_precision, input_scale)` to
/// `(output_precision, output_scale)` and returns the converted number when it fits
/// within the output precision.
///
/// The function first validates that the requested precision and scale are supported for
/// both the source and destination decimal types. It then either upscales (multiplying
/// by an appropriate power of ten) or downscales (dividing with rounding) the input value.
/// When the scaling factor exceeds the precision table of the destination type, the value
/// is treated as an overflow for upscaling, or rounded to zero for downscaling (as any
/// possible result would be zero at the requested scale).
///
/// This mirrors the column-oriented helpers of decimal casting but operates on a single value
/// (row-level) instead of an entire array.
///
/// Returns `None` if the value cannot be represented with the requested precision.
pub fn rescale_decimal<I: DecimalType, O: DecimalType>(
    value: I::Native,
    input_precision: u8,
    input_scale: i8,
    output_precision: u8,
    output_scale: i8,
) -> Option<O::Native>
where
    I::Native: DecimalCast + ArrowNativeTypeOp,
    O::Native: DecimalCast + ArrowNativeTypeOp,
{
    validate_decimal_precision_and_scale::<I>(input_precision, input_scale).ok()?;
    validate_decimal_precision_and_scale::<O>(output_precision, output_scale).ok()?;

    if input_scale <= output_scale {
        let (f, f_infallible) =
            make_upscaler::<I, O>(input_precision, input_scale, output_precision, output_scale)?;
        apply_rescaler::<I, O>(value, output_precision, f, f_infallible)
    } else {
        let Some((f, f_infallible)) =
            make_downscaler::<I, O>(input_precision, input_scale, output_precision, output_scale)
        else {
            // Scale reduction exceeds supported precision; result mathematically rounds to zero
            return Some(O::Native::ZERO);
        };
        apply_rescaler::<I, O>(value, output_precision, f, f_infallible)
    }
}

fn cast_decimal_to_decimal_error<I, O>(
    output_precision: u8,
    output_scale: i8,
) -> impl Fn(<I as ArrowPrimitiveType>::Native) -> ArrowError
where
    I: DecimalType,
    O: DecimalType,
    I::Native: DecimalCast + ArrowNativeTypeOp,
    O::Native: DecimalCast + ArrowNativeTypeOp,
{
    move |x: I::Native| {
        ArrowError::CastError(format!(
            "Cannot cast to {}({}, {}). Overflowing on {:?}",
            O::PREFIX,
            output_precision,
            output_scale,
            x
        ))
    }
}

fn apply_decimal_cast<I: DecimalType, O: DecimalType>(
    array: &PrimitiveArray<I>,
    output_precision: u8,
    output_scale: i8,
    f_fallible: impl Fn(I::Native) -> Option<O::Native>,
    f_infallible: Option<impl Fn(I::Native) -> O::Native>,
    cast_options: &CastOptions,
) -> Result<PrimitiveArray<O>, ArrowError>
where
    I::Native: DecimalCast + ArrowNativeTypeOp,
    O::Native: DecimalCast + ArrowNativeTypeOp,
{
    let array = if let Some(f_infallible) = f_infallible {
        array.unary(f_infallible)
    } else if cast_options.safe {
        array.unary_opt(|x| {
            f_fallible(x).filter(|v| O::is_valid_decimal_precision(*v, output_precision))
        })
    } else {
        let error = cast_decimal_to_decimal_error::<I, O>(output_precision, output_scale);
        array.try_unary(|x| {
            f_fallible(x).ok_or_else(|| error(x)).and_then(|v| {
                O::validate_decimal_precision(v, output_precision, output_scale).map(|_| v)
            })
        })?
    };
    Ok(array)
}

fn convert_to_smaller_scale_decimal<I, O>(
    array: &PrimitiveArray<I>,
    input_precision: u8,
    input_scale: i8,
    output_precision: u8,
    output_scale: i8,
    cast_options: &CastOptions,
) -> Result<PrimitiveArray<O>, ArrowError>
where
    I: DecimalType,
    O: DecimalType,
    I::Native: DecimalCast + ArrowNativeTypeOp,
    O::Native: DecimalCast + ArrowNativeTypeOp,
{
    if let Some((f_fallible, f_infallible)) =
        make_downscaler::<I, O>(input_precision, input_scale, output_precision, output_scale)
    {
        apply_decimal_cast(
            array,
            output_precision,
            output_scale,
            f_fallible,
            f_infallible,
            cast_options,
        )
    } else {
        // Scale reduction exceeds supported precision; result mathematically rounds to zero
        let zeros = vec![O::Native::ZERO; array.len()];
        Ok(PrimitiveArray::new(zeros.into(), array.nulls().cloned()))
    }
}

fn convert_to_bigger_or_equal_scale_decimal<I, O>(
    array: &PrimitiveArray<I>,
    input_precision: u8,
    input_scale: i8,
    output_precision: u8,
    output_scale: i8,
    cast_options: &CastOptions,
) -> Result<PrimitiveArray<O>, ArrowError>
where
    I: DecimalType,
    O: DecimalType,
    I::Native: DecimalCast + ArrowNativeTypeOp,
    O::Native: DecimalCast + ArrowNativeTypeOp,
{
    if let Some((f, f_infallible)) =
        make_upscaler::<I, O>(input_precision, input_scale, output_precision, output_scale)
    {
        apply_decimal_cast(
            array,
            output_precision,
            output_scale,
            f,
            f_infallible,
            cast_options,
        )
    } else {
        // Scale increase exceeds supported precision; return overflow error
        Err(ArrowError::CastError(format!(
            "Cannot cast to {}({}, {}). Value overflows for output scale",
            O::PREFIX,
            output_precision,
            output_scale
        )))
    }
}

// Only support one type of decimal cast operations
pub(crate) fn cast_decimal_to_decimal_same_type<T>(
    array: &PrimitiveArray<T>,
    input_precision: u8,
    input_scale: i8,
    output_precision: u8,
    output_scale: i8,
    cast_options: &CastOptions,
) -> Result<ArrayRef, ArrowError>
where
    T: DecimalType,
    T::Native: DecimalCast + ArrowNativeTypeOp,
{
    let array: PrimitiveArray<T> =
        if input_scale == output_scale && input_precision <= output_precision {
            array.clone()
        } else if input_scale <= output_scale {
            convert_to_bigger_or_equal_scale_decimal::<T, T>(
                array,
                input_precision,
                input_scale,
                output_precision,
                output_scale,
                cast_options,
            )?
        } else {
            // input_scale > output_scale
            convert_to_smaller_scale_decimal::<T, T>(
                array,
                input_precision,
                input_scale,
                output_precision,
                output_scale,
                cast_options,
            )?
        };

    Ok(Arc::new(array.with_precision_and_scale(
        output_precision,
        output_scale,
    )?))
}

// Support two different types of decimal cast operations
pub(crate) fn cast_decimal_to_decimal<I, O>(
    array: &PrimitiveArray<I>,
    input_precision: u8,
    input_scale: i8,
    output_precision: u8,
    output_scale: i8,
    cast_options: &CastOptions,
) -> Result<ArrayRef, ArrowError>
where
    I: DecimalType,
    O: DecimalType,
    I::Native: DecimalCast + ArrowNativeTypeOp,
    O::Native: DecimalCast + ArrowNativeTypeOp,
{
    let array: PrimitiveArray<O> = if input_scale > output_scale {
        convert_to_smaller_scale_decimal::<I, O>(
            array,
            input_precision,
            input_scale,
            output_precision,
            output_scale,
            cast_options,
        )?
    } else {
        convert_to_bigger_or_equal_scale_decimal::<I, O>(
            array,
            input_precision,
            input_scale,
            output_precision,
            output_scale,
            cast_options,
        )?
    };

    Ok(Arc::new(array.with_precision_and_scale(
        output_precision,
        output_scale,
    )?))
}

/// Parses given string to specified decimal native (i128/i256) based on given
/// scale. Returns an `Err` if it cannot parse given string.
pub fn parse_string_to_decimal_native<T: DecimalType>(
    value_str: &str,
    scale: usize,
) -> Result<T::Native, ArrowError>
where
    T::Native: DecimalCast + ArrowNativeTypeOp,
{
    let value_str = value_str.trim();
    let parts: Vec<&str> = value_str.split('.').collect();
    if parts.len() > 2 {
        return Err(ArrowError::InvalidArgumentError(format!(
            "Invalid decimal format: {value_str:?}"
        )));
    }

    let (negative, first_part) = if parts[0].is_empty() {
        (false, parts[0])
    } else {
        match parts[0].as_bytes()[0] {
            b'-' => (true, &parts[0][1..]),
            b'+' => (false, &parts[0][1..]),
            _ => (false, parts[0]),
        }
    };

    let integers = first_part;
    let decimals = if parts.len() == 2 { parts[1] } else { "" };

    if integers.is_empty() && decimals.is_empty() {
        return Err(ArrowError::InvalidArgumentError(format!(
            "Invalid decimal format: {value_str:?}"
        )));
    }

    if !integers.is_empty() && !integers.as_bytes()[0].is_ascii_digit() {
        return Err(ArrowError::InvalidArgumentError(format!(
            "Invalid decimal format: {value_str:?}"
        )));
    }

    if !decimals.is_empty() && !decimals.as_bytes()[0].is_ascii_digit() {
        return Err(ArrowError::InvalidArgumentError(format!(
            "Invalid decimal format: {value_str:?}"
        )));
    }

    // Adjust decimal based on scale
    let mut number_decimals = if decimals.len() > scale {
        let decimal_number = i256::from_string(decimals).ok_or_else(|| {
            ArrowError::InvalidArgumentError(format!("Cannot parse decimal format: {value_str}"))
        })?;

        let div = i256::from_i128(10_i128).pow_checked((decimals.len() - scale) as u32)?;

        let half = div.div_wrapping(i256::from_i128(2));
        let half_neg = half.neg_wrapping();

        let d = decimal_number.div_wrapping(div);
        let r = decimal_number.mod_wrapping(div);

        // Round result
        let adjusted = match decimal_number >= i256::ZERO {
            true if r >= half => d.add_wrapping(i256::ONE),
            false if r <= half_neg => d.sub_wrapping(i256::ONE),
            _ => d,
        };

        let integers = if !integers.is_empty() {
            i256::from_string(integers)
                .ok_or_else(|| {
                    ArrowError::InvalidArgumentError(format!(
                        "Cannot parse decimal format: {value_str}"
                    ))
                })
                .map(|v| v.mul_wrapping(i256::from_i128(10_i128).pow_wrapping(scale as u32)))?
        } else {
            i256::ZERO
        };

        format!("{}", integers.add_wrapping(adjusted))
    } else {
        let padding = if scale > decimals.len() { scale } else { 0 };

        let decimals = format!("{decimals:0<padding$}");
        format!("{integers}{decimals}")
    };

    if negative {
        number_decimals.insert(0, '-');
    }

    let value = i256::from_string(number_decimals.as_str()).ok_or_else(|| {
        ArrowError::InvalidArgumentError(format!(
            "Cannot convert {} to {}: Overflow",
            value_str,
            T::PREFIX
        ))
    })?;

    T::Native::from_decimal(value).ok_or_else(|| {
        ArrowError::InvalidArgumentError(format!("Cannot convert {} to {}", value_str, T::PREFIX))
    })
}

pub(crate) fn generic_string_to_decimal_cast<'a, T, S>(
    from: &'a S,
    precision: u8,
    scale: i8,
    cast_options: &CastOptions,
) -> Result<PrimitiveArray<T>, ArrowError>
where
    T: DecimalType,
    T::Native: DecimalCast + ArrowNativeTypeOp,
    &'a S: StringArrayType<'a>,
{
    if cast_options.safe {
        let iter = from.iter().map(|v| {
            v.and_then(|v| parse_string_to_decimal_native::<T>(v, scale as usize).ok())
                .and_then(|v| T::is_valid_decimal_precision(v, precision).then_some(v))
        });
        // Benefit:
        //     20% performance improvement
        // Soundness:
        //     The iterator is trustedLen because it comes from an `StringArray`.
        Ok(unsafe {
            PrimitiveArray::<T>::from_trusted_len_iter(iter)
                .with_precision_and_scale(precision, scale)?
        })
    } else {
        let vec = from
            .iter()
            .map(|v| {
                v.map(|v| {
                    parse_string_to_decimal_native::<T>(v, scale as usize)
                        .map_err(|_| {
                            ArrowError::CastError(format!(
                                "Cannot cast string '{v}' to value of {} type",
                                T::DATA_TYPE,
                            ))
                        })
                        .and_then(|v| T::validate_decimal_precision(v, precision, scale).map(|_| v))
                })
                .transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;
        // Benefit:
        //     20% performance improvement
        // Soundness:
        //     The iterator is trustedLen because it comes from an `StringArray`.
        Ok(unsafe {
            PrimitiveArray::<T>::from_trusted_len_iter(vec.iter())
                .with_precision_and_scale(precision, scale)?
        })
    }
}

pub(crate) fn string_to_decimal_cast<T, Offset: OffsetSizeTrait>(
    from: &GenericStringArray<Offset>,
    precision: u8,
    scale: i8,
    cast_options: &CastOptions,
) -> Result<PrimitiveArray<T>, ArrowError>
where
    T: DecimalType,
    T::Native: DecimalCast + ArrowNativeTypeOp,
{
    generic_string_to_decimal_cast::<T, GenericStringArray<Offset>>(
        from,
        precision,
        scale,
        cast_options,
    )
}

pub(crate) fn string_view_to_decimal_cast<T>(
    from: &StringViewArray,
    precision: u8,
    scale: i8,
    cast_options: &CastOptions,
) -> Result<PrimitiveArray<T>, ArrowError>
where
    T: DecimalType,
    T::Native: DecimalCast + ArrowNativeTypeOp,
{
    generic_string_to_decimal_cast::<T, StringViewArray>(from, precision, scale, cast_options)
}

/// Cast Utf8 to decimal
pub(crate) fn cast_string_to_decimal<T, Offset: OffsetSizeTrait>(
    from: &dyn Array,
    precision: u8,
    scale: i8,
    cast_options: &CastOptions,
) -> Result<ArrayRef, ArrowError>
where
    T: DecimalType,
    T::Native: DecimalCast + ArrowNativeTypeOp,
{
    if scale < 0 {
        return Err(ArrowError::InvalidArgumentError(format!(
            "Cannot cast string to decimal with negative scale {scale}"
        )));
    }

    if scale > T::MAX_SCALE {
        return Err(ArrowError::InvalidArgumentError(format!(
            "Cannot cast string to decimal greater than maximum scale {}",
            T::MAX_SCALE
        )));
    }

    let result = match from.data_type() {
        DataType::Utf8View => string_view_to_decimal_cast::<T>(
            from.as_any().downcast_ref::<StringViewArray>().unwrap(),
            precision,
            scale,
            cast_options,
        )?,
        DataType::Utf8 | DataType::LargeUtf8 => string_to_decimal_cast::<T, Offset>(
            from.as_any()
                .downcast_ref::<GenericStringArray<Offset>>()
                .unwrap(),
            precision,
            scale,
            cast_options,
        )?,
        other => {
            return Err(ArrowError::ComputeError(format!(
                "Cannot cast {other:?} to decimal",
            )));
        }
    };

    Ok(Arc::new(result))
}

pub(crate) fn cast_floating_point_to_decimal<T: ArrowPrimitiveType, D>(
    array: &PrimitiveArray<T>,
    precision: u8,
    scale: i8,
    cast_options: &CastOptions,
) -> Result<ArrayRef, ArrowError>
where
    <T as ArrowPrimitiveType>::Native: AsPrimitive<f64>,
    D: DecimalType + ArrowPrimitiveType,
    <D as ArrowPrimitiveType>::Native: DecimalCast,
{
    let mul = f64_power_of_ten(scale as i32);

    if cast_options.safe {
        array
            .unary_opt::<_, D>(|v| {
                single_float_to_decimal::<D>(v.as_(), mul)
                    .filter(|v| D::is_valid_decimal_precision(*v, precision))
            })
            .with_precision_and_scale(precision, scale)
            .map(|a| Arc::new(a) as ArrayRef)
    } else {
        array
            .try_unary::<_, D, _>(|v| {
                single_float_to_decimal::<D>(v.as_(), mul)
                    .ok_or_else(|| {
                        ArrowError::CastError(format!(
                            "Cannot cast to {}({}, {}). Overflowing on {:?}",
                            D::PREFIX,
                            precision,
                            scale,
                            v
                        ))
                    })
                    .and_then(|v| D::validate_decimal_precision(v, precision, scale).map(|_| v))
            })?
            .with_precision_and_scale(precision, scale)
            .map(|a| Arc::new(a) as ArrayRef)
    }
}

/// Cast a single floating point value to a decimal native with the given multiple.
/// Returns `None` if the value cannot be represented with the requested precision.
#[inline(always)]
pub fn single_float_to_decimal<D>(input: f64, mul: f64) -> Option<D::Native>
where
    D: DecimalType + ArrowPrimitiveType,
    <D as ArrowPrimitiveType>::Native: DecimalCast,
{
    D::Native::from_f64((mul * input).round())
}

pub(crate) fn cast_decimal_to_integer<D, T>(
    array: &dyn Array,
    scale: i8,
    cast_options: &CastOptions,
) -> Result<ArrayRef, ArrowError>
where
    T: ArrowPrimitiveType,
    <T as ArrowPrimitiveType>::Native: NumCast,
    D: DecimalType + ArrowPrimitiveType,
    <D as ArrowPrimitiveType>::Native: ToPrimitive,
{
    let array = array.as_primitive::<D>();

    let div = D::power_of_ten(scale.unsigned_abs() as u32).ok_or_else(|| {
        ArrowError::CastError(format!(
            "Cannot cast to {:?}. The scale {} causes overflow.",
            D::PREFIX,
            scale,
        ))
    })?;

    let mut value_builder = PrimitiveBuilder::<T>::with_capacity(array.len());

    if scale < 0 {
        match cast_options.safe {
            true => {
                for i in 0..array.len() {
                    if array.is_null(i) {
                        value_builder.append_null();
                    } else {
                        let v = array
                            .value(i)
                            .mul_checked(div)
                            .ok()
                            .and_then(<T::Native as NumCast>::from::<D::Native>);
                        value_builder.append_option(v);
                    }
                }
            }
            false => {
                for i in 0..array.len() {
                    if array.is_null(i) {
                        value_builder.append_null();
                    } else {
                        let v = array.value(i).mul_checked(div)?;

                        let value =
                            <T::Native as NumCast>::from::<D::Native>(v).ok_or_else(|| {
                                ArrowError::CastError(format!(
                                    "value of {:?} is out of range {}",
                                    v,
                                    T::DATA_TYPE
                                ))
                            })?;

                        value_builder.append_value(value);
                    }
                }
            }
        }
    } else {
        match cast_options.safe {
            true => {
                for i in 0..array.len() {
                    if array.is_null(i) {
                        value_builder.append_null();
                    } else {
                        let v = array
                            .value(i)
                            .div_checked(div)
                            .ok()
                            .and_then(<T::Native as NumCast>::from::<D::Native>);
                        value_builder.append_option(v);
                    }
                }
            }
            false => {
                for i in 0..array.len() {
                    if array.is_null(i) {
                        value_builder.append_null();
                    } else {
                        let v = array.value(i).div_checked(div)?;

                        let value =
                            <T::Native as NumCast>::from::<D::Native>(v).ok_or_else(|| {
                                ArrowError::CastError(format!(
                                    "value of {:?} is out of range {}",
                                    v,
                                    T::DATA_TYPE
                                ))
                            })?;

                        value_builder.append_value(value);
                    }
                }
            }
        }
    }
    Ok(Arc::new(value_builder.finish()))
}

/// Cast a decimal array to a floating point array.
///
/// Conversion is lossy and follows standard floating point semantics. Values
/// that exceed the representable range become `INFINITY` or `-INFINITY` without
/// returning an error.
pub(crate) fn cast_decimal_to_float<D: DecimalType, T: ArrowPrimitiveType, F>(
    array: &dyn Array,
    op: F,
) -> Result<ArrayRef, ArrowError>
where
    F: Fn(D::Native) -> T::Native,
{
    let array = array.as_primitive::<D>();
    let array = array.unary::<_, T>(op);
    Ok(Arc::new(array))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_power_of_ten_lookup() {
        for exp in -Decimal256Type::MAX_SCALE as i32..=Decimal256Type::MAX_SCALE as i32 {
            assert_eq!(f64_power_of_ten(exp), 10_f64.powi(exp), "10^{exp}");
        }

        for exp in [-77, 77] {
            assert_eq!(f64_power_of_ten(exp), 10_f64.powi(exp), "10^{exp}");
        }
    }

    #[test]
    fn test_parse_string_to_decimal_native() -> Result<(), ArrowError> {
        assert_eq!(
            parse_string_to_decimal_native::<Decimal128Type>("0", 0)?,
            0_i128
        );
        assert_eq!(
            parse_string_to_decimal_native::<Decimal128Type>("0", 5)?,
            0_i128
        );

        assert_eq!(
            parse_string_to_decimal_native::<Decimal128Type>("123", 0)?,
            123_i128
        );
        assert_eq!(
            parse_string_to_decimal_native::<Decimal128Type>("123", 5)?,
            12300000_i128
        );

        assert_eq!(
            parse_string_to_decimal_native::<Decimal128Type>("123.45", 0)?,
            123_i128
        );
        assert_eq!(
            parse_string_to_decimal_native::<Decimal128Type>("123.45", 5)?,
            12345000_i128
        );

        assert_eq!(
            parse_string_to_decimal_native::<Decimal128Type>("123.4567891", 0)?,
            123_i128
        );
        assert_eq!(
            parse_string_to_decimal_native::<Decimal128Type>("123.4567891", 5)?,
            12345679_i128
        );

        for value in ["", " ", ".", "+", "-", "+.", "-."] {
            assert!(
                parse_string_to_decimal_native::<Decimal128Type>(value, 2).is_err(),
                "expected {value:?} to fail parsing as Decimal128"
            );
            assert!(
                parse_string_to_decimal_native::<Decimal256Type>(value, 2).is_err(),
                "expected {value:?} to fail parsing as Decimal256"
            );
        }
        Ok(())
    }

    #[test]
    fn test_rescale_decimal_upscale_within_precision() {
        let result = rescale_decimal::<Decimal128Type, Decimal128Type>(
            12_345_i128, // 123.45 with scale 2
            5,
            2,
            8,
            5,
        );
        assert_eq!(result, Some(12_345_000_i128));
    }

    #[test]
    fn test_rescale_decimal_downscale_rounds_half_away_from_zero() {
        let positive = rescale_decimal::<Decimal128Type, Decimal128Type>(
            1_050_i128, // 1.050 with scale 3
            5, 3, 5, 1,
        );
        assert_eq!(positive, Some(11_i128)); // 1.1 with scale 1

        let negative = rescale_decimal::<Decimal128Type, Decimal128Type>(
            -1_050_i128, // -1.050 with scale 3
            5,
            3,
            5,
            1,
        );
        assert_eq!(negative, Some(-11_i128)); // -1.1 with scale 1
    }

    #[test]
    fn test_rescale_decimal_downscale_large_delta_returns_zero() {
        let result = rescale_decimal::<Decimal32Type, Decimal32Type>(12_345_i32, 9, 9, 9, 4);
        assert_eq!(result, Some(0_i32));
    }

    #[test]
    fn test_rescale_decimal_upscale_overflow_returns_none() {
        let result = rescale_decimal::<Decimal32Type, Decimal32Type>(9_999_i32, 4, 0, 5, 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_rescale_decimal_invalid_input_precision_scale_returns_none() {
        let result = rescale_decimal::<Decimal128Type, Decimal128Type>(123_i128, 39, 39, 38, 38);
        assert_eq!(result, None);
    }

    #[test]
    fn test_rescale_decimal_invalid_output_precision_scale_returns_none() {
        let result = rescale_decimal::<Decimal128Type, Decimal128Type>(123_i128, 38, 38, 39, 39);
        assert_eq!(result, None);
    }
}
