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

pub struct Vector2<T> { x: T, y: T}

pub struct Vector3<T> { x: T, y: T, z: T}

impl<T:Add<T,T>> Add<Vector2<T>,Vector2<T>> for Vector2<T> {
	fn add(&self, rhs: &Vector2<T>) -> Vector2<T> {
		Vector2{
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
	}
}

impl<T:Sub<T,T>> Sub<Vector2<T>,Vector2<T>> for Vector2<T> {
	fn sub(&self, rhs: &Vector2<T>) -> Vector2<T> {
		Vector2{
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn add_vector2_verification() {
		let v = Vector2{x:1i,y:2i} + Vector2{x:3i,y:5i};
		assert_eq!((v.x,v.y),(4i,7i));
	}

}