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

use std::num::{Zero,One,Algebraic};

// the struct
macro_rules! make_vector_struct(($t:ident $($c:ident),*) => (pub struct $t<T> { $($c: T),*}))

// common fns for all vectors
macro_rules! vec_common_impl(($t:ident $($c:ident),*) => (
    impl<T> $t<T> {
        #[inline] pub fn new($($c:T),*) -> $t<T> {
            $t{
                $(
                    $c: $c
                ),*
            }
        }
    }
))

// zero vectors
macro_rules! vec_zero_impl(($t:ident $($c:ident),*) => (
    impl<T:Zero> Zero for $t<T>{
        fn zero() -> $t<T> {
            $t{
                $(
                    $c: Zero::zero()
                ),*
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
        fn $f(&self, rhs: &$t<T>) -> $t<T> {
            $t{
                $(
                    $c: self.$c.$f(&rhs.$c)
                ),*
            }
        }
    }
))

// negation
macro_rules! vec_negate_impl(($t:ident $($c:ident),*) => (
    impl<T:Neg<T>> Neg<$t<T>> for $t<T> {
        fn neg(&self) -> $t<T> {
            $t{
                $(
                    $c: self.$c.neg()
                ),*
            }
        }
    }
))

// scalar multiplication
macro_rules! vec_scalar_mul_op_imp(($t:ident $($c:ident),*) => (
    impl<T:Mul<T,T>> Mul<T,$t<T>> for $t<T> {
        fn mul(&self, rhs: &T) -> $t<T> {
            $t{
                $(
                    $c: self.$c.mul(rhs)
                ),*
            }
        }
    }
))

macro_rules! vec_general_mul_imp(($t:ident $($c:ident),*) => (
    impl<T:Mul<T,T>> $t<T> {

        fn mul_assign(&mut self, rhs: &T) {
            $(
                self.$c = self.$c.mul(rhs);
            )*
        }
    }
))

// scalar division
macro_rules! vec_scalar_div_op_imp(($t:ident $($c:ident),*) => (
    impl<T:Div<T,T>> Div<T,$t<T>> for $t<T> {
        fn div(&self, rhs: &T) -> $t<T> {
            $t{
                $(
                    $c: self.$c.div(rhs)
                ),*
            }
        }
    }
))

macro_rules! vec_general_div_imp(($t:ident $($c:ident),*) => (
    impl<T:Div<T,T>> $t<T> {

        fn div_assign(&mut self, rhs: &T) {
            $(
                self.$c = self.$c.div(rhs);
            )*
        }
    }
))

// add+mul fn impls
macro_rules! vec_add_mul_op_imp(($t:ident $cf:ident,$($c:ident),*) => (
    impl<T:Mul<T,T>+Add<T,T>> $t<T> {
        pub fn mag_sq(&self) -> T {
            (self.$cf * self.$cf) $(+ (self.$c * self.$c))*
        }

        pub fn dot(&self, rhs: &$t<T>) -> T {
            (self.$cf * rhs.$cf) $(+ (self.$c * rhs.$c))*
        }
    }
))

// mag impls
macro_rules! vec_mag_impl(($t:ident $cf:ident,$($c:ident),*) => (
    impl<T:Mul<T,T>+Add<T,T>+Algebraic> $t<T> {
        pub fn mag(&self) -> T {
            //((self.$cf * self.$cf) $(+ (self.$c * self.$c))*).sqrt()
            self.mag_sq().sqrt()
        }
    }
))

// normals
macro_rules! vec_normal_impl(($t:ident) => (
    impl<T:Div<T,T>+Mul<T,T>+Add<T,T>+Algebraic> $t<T> {
        pub fn normal(&self) -> $t<T>{
            self / self.mag()
        }

        pub fn normalize(&mut self) {
            let m = self.mag();
            self.div_assign(&m);
        }

        pub fn with_magnitude(&self, m:&T) -> $t<T>{
            self * (m / self.mag())
        }

        pub fn set_magnitude(&mut self, m:&T){
            let f = m / self.mag();
            self.mul_assign(&f)
        }

    }
))

// for now this is going to be expanded out just using dumb copy & paste
// later when rust #4375 is fixed this can be cleaned up
// see: https://github.com/mozilla/rust/issues/4375

make_vector_struct!(Vector1 x)
make_vector_struct!(Vector2 x,y)
make_vector_struct!(Vector3 x,y,z)
make_vector_struct!(Vector4 x,y,z,w)

vec_common_impl!(Vector1 x)
vec_common_impl!(Vector2 x,y)
vec_common_impl!(Vector3 x,y,z)
vec_common_impl!(Vector4 x,y,z,w)

vec_zero_impl!(Vector1 x)
vec_zero_impl!(Vector2 x,y)
vec_zero_impl!(Vector3 x,y,z)
vec_zero_impl!(Vector4 x,y,z,w)

vec_simple_op_impl!(Vector1 Add add x)
vec_simple_op_impl!(Vector1 Sub sub x)
vec_simple_op_impl!(Vector2 Add add x,y)
vec_simple_op_impl!(Vector2 Sub sub x,y)
vec_simple_op_impl!(Vector3 Add add x,y,z)
vec_simple_op_impl!(Vector3 Sub sub x,y,z)
vec_simple_op_impl!(Vector4 Add add x,y,z,w)
vec_simple_op_impl!(Vector4 Sub sub x,y,z,w)

vec_negate_impl!(Vector1 x)
vec_negate_impl!(Vector2 x,y)
vec_negate_impl!(Vector3 x,y,z)
vec_negate_impl!(Vector4 x,y,z,w)

vec_scalar_mul_op_imp!(Vector1 x)
vec_scalar_mul_op_imp!(Vector2 x,y)
vec_scalar_mul_op_imp!(Vector3 x,y,z)
vec_scalar_mul_op_imp!(Vector4 x,y,z,w)

vec_general_mul_imp!(Vector1 x)
vec_general_mul_imp!(Vector2 x,y)
vec_general_mul_imp!(Vector3 x,y,z)
vec_general_mul_imp!(Vector4 x,y,z,w)

vec_scalar_div_op_imp!(Vector1 x)
vec_scalar_div_op_imp!(Vector2 x,y)
vec_scalar_div_op_imp!(Vector3 x,y,z)
vec_scalar_div_op_imp!(Vector4 x,y,z,w)

vec_general_div_imp!(Vector1 x)
vec_general_div_imp!(Vector2 x,y)
vec_general_div_imp!(Vector3 x,y,z)
vec_general_div_imp!(Vector4 x,y,z,w)

impl<T:Mul<T,T>> Vector1<T>{ //vec_add_mul_op_imp!(Vector1 x)
    pub fn mag_sq(&self) -> T {self.x * self.x}
    pub fn dot(&self, rhs: &Vector1<T>) -> T { self.x * rhs.x }
}
vec_add_mul_op_imp!(Vector2 x,y)
vec_add_mul_op_imp!(Vector3 x,y,z)
vec_add_mul_op_imp!(Vector4 x,y,z,w)

impl<T:Signed> Vector1<T>{ //vec_mag_impl!(Vector1 x)
    pub fn mag(&self) -> T {self.x.abs()}
}
vec_mag_impl!(Vector2 x,y)
vec_mag_impl!(Vector3 x,y,z)
vec_mag_impl!(Vector4 x,y,z,w)

//vec_normal_impl!(Vector1)
vec_normal_impl!(Vector2)
vec_normal_impl!(Vector3)
vec_normal_impl!(Vector4)

impl<T:Sub<T,T>+Mul<T,T>> Vector2<T>{
    pub fn perp_dot(&self, rhs: &Vector2<T>) -> T {
        (self.x * rhs.y) - (self.y * rhs.x)
    }
}

// unit vectors
// maybe more advanced macro tricks could generate these?
impl<T:One> Vector1<T> {
    pub fn unit() -> Vector1<T> {
        Vector1{
            x: One::one()
        }
    }
}
impl<T:Zero+One> Vector2<T> {
    pub fn x_unit() -> Vector2<T> {
        Vector2{
            x: One::one(),
            y: Zero::zero()
        }
    }
    pub fn y_unit() -> Vector2<T> {
        Vector2{
            x: Zero::zero(),
            y: One::one()
        }
    }
}
impl<T:Zero+One> Vector3<T> {
    pub fn x_unit() -> Vector3<T> {
        Vector3{
            x: One::one(),
            y: Zero::zero(),
            z: Zero::zero()
        }
    }
    pub fn y_unit() -> Vector3<T> {
        Vector3{
            x: Zero::zero(),
            y: One::one(),
            z: Zero::zero()
        }
    }
    pub fn z_unit() -> Vector3<T> {
        Vector3{
            x: Zero::zero(),
            y: Zero::zero(),
            z: One::one()
        }
    }
}
impl<T:Zero+One> Vector4<T> {
    pub fn x_unit() -> Vector4<T> {
        Vector4{
            x: One::one(),
            y: Zero::zero(),
            z: Zero::zero(),
            w: Zero::zero()
        }
    }
    pub fn y_unit() -> Vector4<T> {
        Vector4{
            x: Zero::zero(),
            y: One::one(),
            z: Zero::zero(),
            w: Zero::zero()
        }
    }
    pub fn z_unit() -> Vector4<T> {
        Vector4{
            x: Zero::zero(),
            y: Zero::zero(),
            z: One::one(),
            w: Zero::zero()
        }
    }
    pub fn w_unit() -> Vector4<T> {
        Vector4{
            x: Zero::zero(),
            y: Zero::zero(),
            z: Zero::zero(),
            w: One::one()
        }
    }   
}

#[cfg(test)]
mod tests {
    use std::num::Zero;
    use super::*;

    struct F8_2(f64,f64);
    impl ApproxEq<F8_2> for F8_2 {
        fn approx_epsilon() -> F8_2 {
            F8_2(1.0e-6,1.0e-6)
        }
        fn approx_eq(&self, other: &F8_2) -> bool {
            self.approx_eq_eps(other, &F8_2(1.0e-6,1.0e-6))
        }
        fn approx_eq_eps(&self, other: &F8_2, approx_epsilon: &F8_2) -> bool {
            let F8_2(a,b) = *self;
            let F8_2(c,d) = *other;
            let F8_2(e,f) = *approx_epsilon;
            a.approx_eq_eps(&c,&e) && b.approx_eq_eps(&d,&f)    
        }
    }

    // only testing Vector2 here for most operations
    // the thinking here is that as the impls are generated from macros...
    // if it works for Vector2 it should work for Vector4

    #[test]
    fn new_vector2(){
        let v : Vector2<int> = Vector2::new(1i,2i);
        assert_eq!((v.x,v.y),(1i,2i));
    }

    #[test]
    fn add_vector2() {
        let v = Vector2{x:1i,y:2i} + Vector2{x:3i,y:5i};
        assert_eq!((v.x,v.y),(4i,7i));
    }

    #[test]
    fn sub_vector2() {
        let v = Vector2{x:1i,y:5i} - Vector2{x:3i,y:2i};
        assert_eq!((v.x,v.y),(-2i,3i));
    }

    #[test]
    fn scalar_mul_vector2(){
        let v = Vector2{x:1_f64,y: -3_f64} * 3_f64;
        assert_eq!((v.x,v.y),(3_f64,-9_f64));
    }

    #[test]
    fn mag_vector2(){
        let v = Vector2{x:3_f64, y:4_f64};
        let mut u = Vector2{x:3_f64, y:4_f64};
        u.set_magnitude(&3_f64);
        let t = v.with_magnitude(&2_f64);
        assert_eq!(v.mag_sq(),25_f64);
        assert_eq!(v.mag(),5_f64);
        assert!(F8_2(u.x,u.y).approx_eq(&F8_2(1.8_f64,2.4_f64)))
        assert!(F8_2(t.x,t.y).approx_eq(&F8_2(1.2_f64,1.6_f64)))
    }

    #[test]
    fn zero_vector2(){
        let v : Vector2<int> = Zero::zero();
        assert_eq!((v.x,v.y),(0i,0i));
    }

    #[test]
    fn normal_vector2(){
        let v = Vector2{x: 3_f64,y: 0_f64};
        let n = v.normal();
        assert_eq!((n.x,n.y),(1_f64,0_f64));
    }

    #[test]
    fn dot_vector2(){
        let a = Vector2{x: 5_f64,y: 2_f64};
        let b = Vector2{x: 3_f64,y: -3_f64};
        assert_eq!(a.dot(&b), 9_f64);
    }

    #[test]
    fn perp_dot_vector2(){
        let a = Vector2{x:5i,y:2i};
        let b = Vector2{x:3i,y:-3i};
        assert_eq!(a.perp_dot(&b), -21i)
        assert_eq!(b.perp_dot(&a), 21i)
    }

    #[test]
    fn neg_vector2(){
        let v = Vector2{x: 1i, y:-2i}.neg();
        assert_eq!((v.x,v.y),(-1i,2i));
    }

    #[test]
    fn unit_vector1() {
        let u : Vector1<f64> = Vector1::unit();
        assert_eq!(u.x,1_f64);
    }

    #[test]
    fn unit_vector2() {
        let x : Vector2<f64> = Vector2::x_unit();
        let y : Vector2<int> = Vector2::y_unit();
        assert_eq!((x.x,x.y),(1_f64,0_f64));
        assert_eq!((y.x,y.y),(0i,1i));
    }

    #[test]
    fn unit_vector3() {
        let x : Vector3<f64> = Vector3::x_unit();
        let y : Vector3<int> = Vector3::y_unit();
        let z : Vector3<int> = Vector3::z_unit();
        assert_eq!((x.x,x.y,x.z),(1_f64,0_f64,0_f64));
        assert_eq!((y.x,y.y,y.z),(0i,1i,0i));
        assert_eq!((z.x,z.y,z.z),(0i,0i,1i));
    }

    #[test]
    fn unit_vector4() {
        let x : Vector4<f64> = Vector4::x_unit();
        let y : Vector4<int> = Vector4::y_unit();
        let z : Vector4<int> = Vector4::z_unit();
        let w : Vector4<int> = Vector4::w_unit();
        assert_eq!((x.x,x.y,x.z,x.w),(1_f64,0_f64,0_f64,0_f64));
        assert_eq!((y.x,y.y,y.z,y.w),(0i,1i,0i,0i));
        assert_eq!((z.x,z.y,z.z,z.w),(0i,0i,1i,0i));
        assert_eq!((w.x,w.y,w.z,w.w),(0i,0i,0i,1i));
    }

}