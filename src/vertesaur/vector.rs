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

macro_rules! vec_op_impl(($t:ident $i:ident $f:ident $($c:ident),*) => (
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

macro_rules! build_vectors(($({$t:ident $($c:ident),*})|*) => (
	mod generated{$(
		pub struct $t<T> { $($c: T,)*}
		vec_op_impl!($t Add add $($c),*)
		vec_op_impl!($t Sub sub $($c),*)
	)*}
))


build_vectors!(
	{Vector2 x,y}
	|{Vector3 x,y,z}
	|{Vector4 x,y,z,w}
)

#[cfg(test)]
mod tests {
	use super::generated::*;

	// the thinking here is that as the impls are generated from macros...
	// if it works for Vector2 it should work for Vector4

	#[test]
	fn add_vector2_verification() {
		let v = Vector2{x:1i,y:2i} + Vector2{x:3i,y:5i};
		assert_eq!((v.x,v.y),(4i,7i));
	}

}