// Copyright (c) 2013 Aaron Dandy
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use std::num::{Zero,One};

macro_rules! make_vector_struct(($t:ident $($c:ident),*) => (pub struct $t<T> { $($c: T),*}))

macro_rules! vec_zero_impl(($t:ident $($c:ident),*) => (
	impl<T:Zero> Zero for $t<T>{
		fn zero() -> $t<T> {
			$t{
				$(
					$c: Zero::zero(),
				)*
			}
		}

		fn is_zero(&self) -> bool {
			false // I have no idea how to compare values!
		}
	}
))

// simple component to vector operator abstractions
macro_rules! vec_simple_op_impl(($t:ident $i:ident $f:ident $($c:ident),*) => (
	impl<T:$i<T,T>> $i<$t<T>,$t<T>> for $t<T> {
		#[inline] fn $f(&self, rhs: &$t<T>) -> $t<T> {
			$t{
				$(
					$c: self.$c.$f(&rhs.$c),
				)*
			}
		}
	}
))

// scalar multiplication
macro_rules! vec_scalar_mul_op_imp(($t:ident $($c:ident),*) => (
	impl<T:Mul<T,T>> Mul<T,$t<T>> for $t<T> {
		#[inline] fn mul(&self, rhs: &T) -> $t<T> {
			$t{
				$(
					$c: self.$c.mul(rhs),
				)*
			}
		}
	}
))

// for now this is going to be expanded out just using dumb copy & paste
// later when rust #4375 is fixed this can be cleaned up
// see: https://github.com/mozilla/rust/issues/4375

make_vector_struct!(Vector1 x)
vec_simple_op_impl!(Vector1 Add add x)
vec_simple_op_impl!(Vector1 Sub sub x)
vec_scalar_mul_op_imp!(Vector1 x)

make_vector_struct!(Vector2 x,y)
vec_simple_op_impl!(Vector2 Add add x,y)
vec_simple_op_impl!(Vector2 Sub sub x,y)
vec_scalar_mul_op_imp!(Vector2 x,y)

make_vector_struct!(Vector3 x,y,z)
vec_simple_op_impl!(Vector3 Add add x,y,z)
vec_simple_op_impl!(Vector3 Sub sub x,y,z)
vec_scalar_mul_op_imp!(Vector3 x,y,z)

make_vector_struct!(Vector4 x,y,z,w)
vec_simple_op_impl!(Vector4 Add add x,y,z,w)
vec_simple_op_impl!(Vector4 Sub sub x,y,z,w)
vec_scalar_mul_op_imp!(Vector4 x,y,z,w)

vec_zero_impl!(Vector2 x,y)

impl<T:Zero+One> Vector2<T> {
	fn x_unit() -> Vector2<T> {
		Vector2{
			x: One::one(),
			y: Zero::zero()
		}
	}

	fn y_unit() -> Vector2<T> {
		Vector2{
			x: Zero::zero(),
			y: One::one()
		}
	}

}

impl<T> Vector2<T> {
	fn some_garbage(&self) -> bool {
		false
	}

}

#[cfg(test)]
mod tests {
	use std::num::Zero;
	use super::*;

	// only testing Vector2 here for most operations
	// the thinking here is that as the impls are generated from macros...
	// if it works for Vector2 it should work for Vector4

	#[test]
	fn add_vector2_verification() {
		let v = Vector2{x:1i,y:2i} + Vector2{x:3i,y:5i};
		assert_eq!((v.x,v.y),(4i,7i));
	}

	#[test]
	fn sub_vector2_verification() {
		let v = Vector2{x:1i,y:5i} - Vector2{x:3i,y:2i};
		assert_eq!((v.x,v.y),(-2i,3i));
	}

	#[test]
	fn scalar_mul_vector2_verification(){
		let v = Vector2{x:1_f64,y: -3_f64} * 3_f64;
		assert_eq!((v.x,v.y),(3_f64,-9_f64));
	}

	#[test]
	fn zero_vector_verification(){
		let v : Vector2<int> = Zero::zero();
		assert_eq!((v.x,v.y),(0i,0i));
	}

	#[test]
	fn unit_vector2_verify() {
		let x : Vector2<f64> = Vector2::x_unit();
		let y : Vector2<int> = Vector2::y_unit();
		let b = x.some_garbage();
		assert_eq!((x.x,x.y),(1_f64,0_f64));
		assert_eq!((y.x,y.y),(0i,1i));
	}

}