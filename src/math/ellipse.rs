use crate::{
    Abs, Angle, Atan, Atan2, Complex, Cos, Cube, DeltaAngle, FromDurationAsSecs, FromSecs, IsNan,
    IsNeg, Line, NonNeg, One, Pi, Point, RemEuclid, Sin, Sq, Sqrt, Tan, Two, UnsignedContstant,
    Vector, Zero, lerp, physics::Kg, time::RelativeDuration, uconst,
};
use core::{
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse<T> {
    x: T,
    y: T,
    a: T,
    b: T,
    r: T,
    i: T,
}

impl<T> Ellipse<T> {
    pub fn center(&self) -> Point<T>
    where
        T: Clone,
    {
        (self.x.clone(), self.y.clone()).into()
    }

    pub fn axes(&self) -> Vector<T>
    where
        T: Clone,
    {
        (self.a.clone(), self.b.clone()).into()
    }

    pub fn rotation(&self) -> Complex<T>
    where
        T: Clone,
    {
        (self.r.clone(), self.i.clone()).into()
    }

    pub fn with_center(self, c: Point<T>) -> Self {
        let (x, y) = c.into();
        Self {
            x,
            y,
            a: self.a,
            b: self.b,
            r: self.r,
            i: self.i,
        }
    }

    pub fn with_axes(self, a: Vector<T>) -> Self {
        let (a, b) = a.into();
        Self {
            x: self.x,
            y: self.y,
            a,
            b,
            r: self.r,
            i: self.i,
        }
    }

    pub fn with_rotation(self, c: Complex<T>) -> Self {
        let (r, i) = c.into();
        Self {
            x: self.x,
            y: self.y,
            a: self.a,
            b: self.b,
            r,
            i,
        }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn a(&self) -> &T {
        &self.a
    }

    pub fn b(&self) -> &T {
        &self.b
    }

    pub fn r(&self) -> &T {
        &self.r
    }

    pub fn i(&self) -> &T {
        &self.i
    }

    /// Into world space
    fn into_ws(&self, p: Point<T>) -> Point<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone + Zero,
    {
        (p.rotated(Point::origin(), self.rotation())).absolute(self.center())
    }

    /// From world space
    fn from_ws(&self, p: Point<T>) -> Point<T>
    where
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Neg<Output = T>
            + Sq<Output = NonNeg<T>>
            + Clone
            + Zero,
    {
        p.relative(self.center())
            .rotated(Point::origin(), !self.rotation())
    }

    pub const fn from_raw(x: T, y: T, a: T, b: T, r: T, i: T) -> Self {
        Self { x, y, a, b, r, i }
    }

    pub fn new(center: Point<T>, axes: Vector<T>, rotation: Complex<T>) -> Self {
        let (x, y) = center.into();
        let (a, b) = axes.into();
        let (r, i) = rotation.into();
        Self { x, y, a, b, r, i }
    }

    pub fn from_angle(center: Point<T>, axes: Vector<T>, rotation: Angle<T>) -> Self
    where
        T: Sin<Output = T> + Cos<Output = T> + Clone,
    {
        let (x, y) = center.into();
        let (a, b) = axes.into();
        Self {
            x,
            y,
            a,
            b,
            r: rotation.clone().cos(),
            i: rotation.sin(),
        }
    }

    /// Focal point 0. Returns x, y of the point
    pub fn f0_old(&self) -> Point<T>
    where
        T: Abs<Output = NonNeg<T>>
            + PartialOrd
            + IsNan
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Sq<Output = NonNeg<T>>
            + Sqrt<Output = T>
            + Clone
            + Zero,
    {
        if self.a.clone().abs() > self.b.clone().abs() {
            let major_axis = absmax(self.a.clone(), self.b.clone());
            let minor_axis = absmin(self.a.clone(), self.b.clone());
            let focal_len = (major_axis.sq() - minor_axis.sq()).sqrt();

            self.into_ws((T::zero(), focal_len).into())
        } else {
            let major_axis = absmax(self.a.clone(), self.b.clone());
            let minor_axis = absmin(self.a.clone(), self.b.clone());
            let focal_len = (major_axis.sq() - minor_axis.sq()).sqrt();

            self.into_ws((focal_len, T::zero()).into())
        }
    }

    pub fn f0(&self) -> Point<T>
    where
        T: Abs<Output = NonNeg<T>>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Sq<Output = NonNeg<T>>
            + PartialOrd
            + Clone
            + IsNeg
            + Zero,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        if self.a.clone().abs() > self.b.clone().abs() {
            let focal_len = (NonNeg::new(self.a.clone().sq() - self.b.clone().sq())
                .ok()
                .unwrap())
            .sqrt();
            self.into_ws((T::zero(), focal_len.into_inner()).into())
        } else {
            let focal_len = (NonNeg::new(self.b.clone().sq() - self.a.clone().sq())
                .ok()
                .unwrap())
            .sqrt();
            self.into_ws((focal_len.into_inner(), T::zero()).into())
        }
    }

    /// Focal point 1. Returns x, y of the point
    pub fn f1(&self) -> Point<T>
    where
        T: Abs<Output = NonNeg<T>>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Neg<Output = T>
            + Sq<Output = NonNeg<T>>
            + PartialOrd
            + IsNeg
            + IsNan
            + Clone
            + Zero,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        if self.a.clone().abs() > self.b.clone().abs() {
            let major_axis = absmax(self.a.clone(), self.b.clone());
            let minor_axis = absmin(self.a.clone(), self.b.clone());
            let focal_len = NonNeg::new(major_axis.sq() - minor_axis.sq())
                .ok()
                .unwrap()
                .sqrt();
            self.into_ws((Zero::zero(), -focal_len.into_inner()).into())
        } else {
            let major_axis = absmax(self.a.clone(), self.b.clone());
            let minor_axis = absmin(self.a.clone(), self.b.clone());
            let focal_len = NonNeg::new(major_axis.sq() - minor_axis.sq())
                .ok()
                .unwrap()
                .sqrt();
            self.into_ws((-focal_len.into_inner(), Zero::zero()).into())
        }
    }

    pub fn from_foci(f0: Point<T>, f1: Point<T>, point_on_ellipse: Point<T>) -> Self
    where
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Sq<Output = NonNeg<T>>
            + Neg<Output = T>
            + Cos<Output = T>
            + Sin<Output = T>
            + IsNeg
            + One
            + Two
            + Clone
            + Pi,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        let c = (f0.clone() - f1.clone()).len().into_inner() / T::two();

        // (sum / 2.)^2 = c^2 + b^2;
        // sum = (a-c)*2 + c*2
        // sum = a*2
        // a^2 == c^2 + b^2;

        let sum = (f0.clone() - point_on_ellipse.clone()).len().into_inner()
            + (f1.clone() - point_on_ellipse).len().into_inner();
        let a = sum / T::two();
        let b = NonNeg::new(a.clone().sq() - c.sq()).ok().unwrap().sqrt();

        let center = lerp(f0.clone(), f1, T::one() / T::two());

        let rot: Complex<T> = (f0 - center.clone()).rotor();

        let rot: Complex<T> =
            rot * Complex::from_polar(T::one(), Angle::from_radians(-T::pi() / T::two()));

        let (x, y) = center.into();
        let (r, i) = rot.into();
        Ellipse {
            x,
            y,
            a,
            b: b.into_inner(),
            r,
            i,
        }
    }

    pub fn radius(&self, anomaly: Angle<T>) -> T
    where
        T: Add<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Sq<Output = NonNeg<T>>
            + Cos<Output = T>
            + Sin<Output = T>
            + Clone,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        let a = self.a.clone();
        let b = self.b.clone();
        a.clone() * b.clone()
            / ((b * anomaly.clone().cos()).sq() + (a * anomaly.sin()).sq())
                .sqrt()
                .into_inner()
    }

    pub fn apoapsis(&self) -> NonNeg<T>
    where
        T: Abs<Output = NonNeg<T>>
            + Add<Output = T>
            + Sub<Output = T>
            + PartialOrd
            + Sq<Output = NonNeg<T>>
            + Clone
            + IsNeg,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        if self.a.clone().abs() > self.b.clone().abs() {
            let focal_len = NonNeg::new(self.a.clone().sq() - self.b.clone().sq())
                .ok()
                .unwrap()
                .sqrt();
            self.a.clone().abs() + focal_len
        } else {
            let focal_len = NonNeg::new(self.b.clone().sq() - self.a.clone().sq())
                .ok()
                .unwrap()
                .sqrt();
            self.b.clone().abs() + focal_len
        }
    }

    pub fn periapsis(&self) -> T
    where
        T: Abs<Output = NonNeg<T>>
            + Sub<Output = T>
            + Sq<Output = NonNeg<T>>
            + PartialOrd
            + Clone
            + IsNeg,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        if self.a.clone().abs() > self.b.clone().abs() {
            let focal_len = NonNeg::new(self.a.clone().sq() - self.b.clone().sq())
                .ok()
                .unwrap()
                .sqrt();
            self.a.clone().abs() - focal_len
        } else {
            let focal_len = NonNeg::new(self.b.clone().sq() - self.a.clone().sq())
                .ok()
                .unwrap()
                .sqrt();
            self.b.clone().abs() - focal_len
        }
    }

    pub fn point_on_ellipse(&self, anomaly: Angle<T>) -> Point<T>
    where
        T: Abs<Output = NonNeg<T>>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Cos<Output = T>
            + Sin<Output = T>
            + Clone
            + Zero,
    {
        self.into_ws(
            (
                self.b.clone().abs().into_inner() * anomaly.clone().sin(),
                self.a.clone().abs().into_inner() * anomaly.cos(),
            )
                .into(),
        )
    }

    /// Returns anomaly of point on ellipse. Precision depends on how close the point is to the ellipse circumference
    pub fn anomaly(&self, position: Point<T>) -> Angle<T>
    where
        T: Atan2<Output = T>
            + Abs<Output = NonNeg<T>>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Neg<Output = T>
            + Sq<Output = NonNeg<T>>
            + PartialOrd
            + Clone
            + IsNeg
            + Zero
            + Two
            + Pi,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        let (x, y) = self.from_ws(position).into();

        let a = T::atan2(
            x / self.b.clone().abs().into_inner(),
            y / self.a.clone().abs().into_inner(),
        );

        // Angle::from_radians(a)
        a

        // vec.angle()+ DeltaAngle::<T>::pi() / T::two()
    }

    pub fn acc(
        &self,
        anomaly: Angle<T>,
        central_body_mass: Kg<T>,
        gravitational_constant: T,
    ) -> Vector<T>
    where
        T: Cube<Output = T>
            + Abs<Output = NonNeg<T>>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Cos<Output = T>
            + Sin<Output = T>
            + Sq<Output = NonNeg<T>>
            + PartialOrd
            + IsNeg
            + Clone
            + Zero,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        let vec = self.f0() - self.point_on_ellipse(anomaly);
        let vec_len = vec.clone().len().into_inner();
        vec * central_body_mass.0 * gravitational_constant / vec_len.cube()
    }

    pub fn tangential_velocity(
        &self,
        anomaly: Angle<T>,
        central_body_mass: Kg<T>,
        gravitational_constant: T,
    ) -> Option<Vector<T>>
    where
        T: Abs<Output = NonNeg<T>>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Cos<Output = T>
            + Sin<Output = T>
            + Neg<Output = T>
            + Sq<Output = NonNeg<T>>
            + PartialOrd
            + Clone
            + IsNeg
            + Zero,
        NonNeg<T>: Sqrt<Output = NonNeg<T>> + One + Two,
    {
        // let p = self.point_on_ellipse(t);
        // let center = (self.x, self.y);

        let vec = self.f0() - self.point_on_ellipse(anomaly.clone());
        let vec_len = vec.len();

        // let radius = self.radius(t);
        // println!("radius: {}", radius);

        let velocity_module = NonNeg::new(
            gravitational_constant
                * central_body_mass.0
                * (NonNeg::two() / vec_len - NonNeg::one() / self.a.clone().abs()),
        )
        .ok()?
        .sqrt();

        // let tangent = (
        //     radius*T::sin(t * 2. * PI),
        //     radius*T::cos(t * 2. * PI),
        // );

        // let tangent = vmath::complex_mul( tangent, (self.i, self.r));

        let vel = Vector::from((
            // radius * T::sin(t * 2. * PI + PI/2.),
            // radius * T::cos(t * 2. * PI + PI/2.),
            self.b.clone().abs().into_inner() * anomaly.clone().cos(),
            self.a.clone().abs().into_inner() * -anomaly.sin(),
        )) * self.rotation();

        Some(vel.norm() * velocity_module.into_inner())
    }

    pub fn angular_velocity(
        &self,
        anomaly: Angle<T>,
        central_body_mass: Kg<T>,
        gravitational_constant: T,
    ) -> Option<DeltaAngle<T>>
    where
        T: Abs<Output = NonNeg<T>>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Cos<Output = T>
            + Sin<Output = T>
            + Neg<Output = T>
            + Sq<Output = NonNeg<T>>
            + PartialOrd
            + Clone
            + IsNeg
            + Zero,
        NonNeg<T>: Sqrt<Output = NonNeg<T>> + One + Two,
    {
        let p = self.point_on_ellipse(anomaly.clone());
        let f0 = self.f0();
        let r = (p - f0).len().into_inner();
        let v = self
            .tangential_velocity(anomaly, central_body_mass, gravitational_constant)?
            .len()
            .into_inner();

        Some(DeltaAngle::from_radians(v / r))
    }

    pub fn f1_from_tangential_velocity(
        &self,
        anomaly: Angle<T>,
        central_body_mass: Kg<T>,
        gravitational_constant: T,
        vel: Vector<T>,
    ) -> (Vector<T>, Point<T>)
    where
        T: Abs<Output = NonNeg<T>>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Sq<Output = NonNeg<T>>
            + Cos<Output = T>
            + Sin<Output = T>
            + Neg<Output = T>
            + PartialOrd
            + Clone
            + IsNeg
            + Zero
            + One
            + Two,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        let p = self.point_on_ellipse(anomaly);
        let f0 = self.f0();

        let _r = p - f0.clone();
        let _v = vel;
        let _mu = central_body_mass.0 * gravitational_constant;
        let _r_len = _r.clone().len().into_inner();
        let _v_len = _v.clone().len().into_inner();
        let _h = _r.clone().cross(_v.clone());
        let _energy = _v_len.clone().sq().into_inner() / T::two() - _mu.clone() / _r_len.clone();
        let _a = -_mu.clone() / (T::two() * _energy);
        let _e = (_r.clone() * (_v_len.sq().into_inner() - _mu.clone() / _r_len)
            - _v.clone() * _r.dot(_v))
            * (T::one() / _mu);
        let _f1 = _e.clone() * -T::two() * _a;

        (_e, f0 + _f1)
    }

    pub fn perimeter(&self) -> T
    where
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Sq<Output = NonNeg<T>>
            + IsNeg
            + Pi
            + One
            + UnsignedContstant<3>
            + UnsignedContstant<4>
            + UnsignedContstant<10>
            + Clone,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        let h = (self.a.clone() - self.b.clone()).sq() / (self.a.clone() + self.b.clone()).sq();
        T::pi()
            * (self.a.clone() + self.b.clone())
            * (T::one()
                + uconst::<3, T>() * h.clone().into_inner()
                    / (uconst::<10, T>()
                        + NonNeg::new(uconst::<4, T>() - uconst::<3, T>() * h.into_inner())
                            .ok()
                            .unwrap()
                            .sqrt()
                            .into_inner()))
    }

    /// Change ellipse in the way that f0, and position in `t` stays the same and velocity in `t` changeds to `vel`
    pub fn set_tangential_velicity(
        &self,
        anomaly: Angle<T>,
        central_body_mass: Kg<T>,
        gravitational_constant: T,
        vel: Vector<T>,
    ) -> (T, T, T)
    where
        T: RemEuclid<Output = NonNeg<T>>
            + Abs<Output = NonNeg<T>>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Cos<Output = T>
            + Sin<Output = T>
            + Neg<Output = T>
            + Sq<Output = NonNeg<T>>
            + PartialOrd
            + Clone
            + IsNeg
            + Zero
            + Pi
            + Two
            + One
            + UnsignedContstant<4>,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        // consts
        let four = uconst::<4, T>();
        let p = self.point_on_ellipse(anomaly.clone());
        let f0 = self.f0();
        let vec_to_focus = f0.clone() - p.clone();
        #[allow(unused)]
        let vec_to_focus_len = vec_to_focus.len().into_inner();
        #[allow(unused)]
        let th = anomaly.radians().into_inner();
        #[allow(unused)]
        let velocity_module = vel.clone().len().into_inner();
        #[allow(unused)]
        let q = T::one()
            / (four.clone() / vec_to_focus_len
                - velocity_module.sq().into_inner() / gravitational_constant / central_body_mass.0);
        // ------

        // #[allow(unused)]
        // let (new_a, new_b, new_center, new_rotation): (T, T, Point<T>, Complex<T>) = todo!();

        /*

        #[allow(unreachable_code)]
        if new_a.abs() > new_b.abs() {
            assert!(
                new_center
                    == f0 - Vector::from((0., (q.sq() - new_b.sq()).sqrt())) * new_rotation
            );

            assert!(
                new_rotation
                    == Complex::div(
                        p - f0,
                        Vector::from((
                            new_b.abs() * T::sin(th),
                            q.abs() * T::cos(th) - (q.sq() - new_b.sq()).sqrt(),
                        ))
                    )
            ); // 3

            /*

                    {(a * c + b * d) / (c^2 + d^2) ,
                    (b * c - a * d) / (c^2 + d^2) }


                {
                    (new_b.abs() * T::cos(th) * new_b.abs() * T::sin(th) + q.abs() * -T::sin(th) * (q.abs() * T::cos(th) - (q.sq() - new_b.sq()).sqrt())) / ((new_b.abs() * T::sin(th))^2 + (q.abs() * T::cos(th) - (q.sq() - new_b.sq()).sqrt())^2) ,
                    (q.abs() * -T::sin(th) * new_b.abs() * T::sin(th) - new_b.abs() * T::cos(th) * (q.abs() * T::cos(th) - (q.sq() - new_b.sq()).sqrt())) / ((new_b.abs() * T::sin(th))^2 + (q.abs() * T::cos(th) - (q.sq() - new_b.sq()).sqrt())^2)
                }

                {
                    (
                          new_b^2 * cos(th) * sin(th)
                        - q^2 * cos(th) * sin(th)
                        + |q| * sin(th) * sqrt(q^2 - new_b^2)
                    )

                        / (
                            new_b^2 * sin(th)^2
                            - 2*|q| * cos(th) * sqrt(q^2 - new_b^2)
                            - new_b^2
                            + 2*q^2 * cos(th)^2
                        ) ,

                    (|q| * -sin(th) * |new_b| * sin(th) - |new_b| * cos(th) * (|q| * cos(th) - (q^2 - new_b^2).sqrt())) / ((|new_b| * sin(th))^2 + (|q| * cos(th) - (q^2 - new_b^2).sqrt())^2)
                }



                                    (
                          new_b^2 * cos(th) * sin(th)
                        - q^2 * cos(th) * sin(th)
                        + |q| * sin(th) * sqrt(q^2 - new_b^2)
                    )

                        / (
                            new_b^2 * sin(th)^2
                            - 2*|q| * cos(th) * sqrt(q^2 - new_b^2)
                            - new_b^2
                            + 2*q^2 * cos(th)^2
                        ) == (vel.x *(p.x-f0.x) + vel.y * (p.y-f0.y)) / ((p.x-f0.x)^2 + (p.y-f0.y)^2)





                         {(vel.x *(p.x-f0.x) + vel.y * (p.y-f0.y)) / ((p.x-f0.x)^2 + (p.y-f0.y)^2) ,
                    (vel.y * (p.x-f0.x) - vel.x * (p.y-f0.y)) / ((p.x-f0.x)^2 + (p.y-f0.y)^2) }

            */

            assert!(

                   com(
                         new_b.abs() * T::cos(th),
                         q.abs() * -T::sin(th)
                      )

                 / com(
                         new_b.abs() * T::sin(th),
                         q.abs() * T::cos(th) - (q.sq() - new_b.sq()).sqrt()
                      )

                == com(vel) / com(p - f0)

            );


        } else {
            assert!(self.into_ws(((new_b.sq() - q.sq()).sqrt(), 0.).into()) == f0);

            assert!(
                self.into_ws((new_b.abs() * T::sin(th), q.abs() * T::cos(th),).into()) == p
            );

            assert!(
                (Vector::from((new_b.abs() * T::cos(th), q.abs() * -T::sin(th),))
                    * new_rotation)
                    .norm()
                    == vel.norm()
            );
        }

        */

        let sin = |x| T::sin(x);
        let cos = |x| T::cos(x);

        let hx = (vel.x().clone() * (p.x().clone() - f0.x().clone())
            + vel.y().clone() * (p.y().clone() - f0.y().clone()))
            / ((p.x().clone() - f0.x().clone()).sq().into_inner()
                + (p.y().clone() - f0.y().clone()).sq().into_inner());
        let jx = hx.clone() * sin(th.clone()).sq().into_inner()
            - cos(th.clone()) * sin(th.clone())
            - hx.clone();
        let lx =
            hx.clone() * T::two() * q.clone().sq().into_inner() * cos(th.clone()).sq().into_inner()
                + q.clone().sq().into_inner() * cos(th.clone()) * sin(th.clone());
        let ox = (q.clone().abs().into_inner() * sin(th.clone())
            + hx * T::two() * q.clone().abs().into_inner() * cos(th))
        .sq();

        // let xxxx =
        //                jx^2 * new_b^4
        //              + (ox + 2 * jx * lx) * new_b^2
        //              - q^2 * ox - lx^2
        //              == 0

        let desc = NonNeg::new(
            (ox.clone().into_inner() + T::two() * jx.clone() * lx.clone())
                .sq()
                .into_inner()
                + four
                    * jx.clone().sq().into_inner()
                    * (q.clone().sq().into_inner() * ox.clone().into_inner()
                        + lx.clone().sq().into_inner()),
        )
        .ok()
        .unwrap();

        let new_b0 = (-(ox.clone().into_inner() + T::two() * jx.clone() * lx.clone())
            + desc.clone().sqrt().into_inner())
            / (T::two() * jx.clone().sq().into_inner());
        let new_b1 = (-(ox.into_inner() + T::two() * jx.clone() * lx) - desc.sqrt().into_inner())
            / (T::two() * jx.sq().into_inner());

        (q, new_b0, new_b1)
    }

    pub fn accelerated(
        &self,
        anomaly: Angle<T>,
        central_body_mass: Kg<T>,
        gravitational_constant: T,
        dt: Duration,
        acc: Vector<T>,
    ) -> Self
    where
        T: Abs<Output = NonNeg<T>>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Cos<Output = T>
            + Sin<Output = T>
            + Neg<Output = T>
            + Sq<Output = NonNeg<T>>
            + PartialOrd
            + FromDurationAsSecs
            + Clone
            + IsNeg
            + Zero
            + One
            + Two
            + Pi,
        NonNeg<T>: Sqrt<Output = NonNeg<T>> + One + Two,
    {
        let f0 = self.f0();
        let p = self.point_on_ellipse(anomaly.clone());
        let vel = self
            .tangential_velocity(
                anomaly.clone(),
                central_body_mass.clone(),
                gravitational_constant.clone(),
            )
            .unwrap();
        let (_, new_f1) = self.f1_from_tangential_velocity(
            anomaly,
            central_body_mass,
            gravitational_constant,
            vel + acc * T::from_duration_as_secs(dt),
        );
        Ellipse::from_foci(f0, new_f1, p)
    }

    pub fn eccentricity(&self) -> NonNeg<T>
    where
        T: Sub<Output = T> + Div<Output = T> + Sq<Output = NonNeg<T>> + IsNeg + Clone + One,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        NonNeg::new(T::one() - (self.b.clone() / self.a.clone()).sq().into_inner())
            .ok()
            .unwrap()
            .sqrt()
    }

    pub fn time_between_anomalies(
        &self,
        anomaly0: Angle<T>,
        anomaly1: Angle<T>,
        central_body_mass: Kg<T>,
        gravitational_constant: T,
    ) -> Duration
    where
        T: RemEuclid<Output = NonNeg<T>>
            + Cube<Output = T>
            + Atan<Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Sin<Output = T>
            + Tan<Output = T>
            + Sq<Output = NonNeg<T>>
            + AddAssign
            + Clone
            + IsNeg
            + One
            + Two
            + Pi,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
        Duration: FromSecs<T>,
    {
        time_between_true_anomalies(
            self.a.clone(),
            self.eccentricity(),
            anomaly0,
            anomaly1,
            central_body_mass.0 * gravitational_constant,
        )
    }

    /// Unlike `time_between_anomalies` can return negative time
    pub fn relative_time_between_anomalies(
        &self,
        anomaly0: Angle<T>,
        anomaly1: Angle<T>,
        central_body_mass: Kg<T>,
        gravitational_constant: T,
    ) -> RelativeDuration
    where
        T: RemEuclid<Output = NonNeg<T>>
            + Cube<Output = T>
            + Atan<Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Sin<Output = T>
            + Tan<Output = T>
            + Sq<Output = NonNeg<T>>
            + IsNeg
            + Clone
            + One
            + Two
            + Pi,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
        RelativeDuration: FromSecs<T>,
    {
        relative_time_between_true_anomalies(
            self.a.clone(),
            self.eccentricity(),
            anomaly0,
            anomaly1,
            central_body_mass.0 * gravitational_constant,
        )
    }

    pub fn eq(&self) -> impl FnOnce(Point<T>) -> T
    where
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Sq<Output = NonNeg<T>>
            + Clone
            + One,
    {
        let x_0 = self.x.clone();
        let y_0 = self.y.clone();
        let a = self.a.clone();
        let b = self.b.clone();
        let r = self.r.clone();
        let i = self.i.clone();
        move |point| {
            let (x, y) = point.into();
            ((r.clone() * (x.clone() - x_0.clone()) - i.clone() * (y.clone() - y_0.clone())).sq()
                / a.sq()
                + (r * (y - y_0) + i * (x - x_0)).sq() / b.sq())
            .into_inner()
                - T::one()
        }
    }

    /// (r*(x - x_0) - i*(y - y_0))^2 / a^2 + (r*(y - y_0) + i*(x - x_0))^2 / b^2 - 1
    /// y = k*x+d
    ///
    /// (r * (x - x_0) - i * (k * x + d - y_0))^2 / a^2 + (r * (k * x + d - y_0) + i * (x - x_0))^2 / b^2 - 1
    /// (r * (x - x_0) - e * (k * x + d - y_0))^2 / a^2 + (r * (k * x + d - y_0) + e * (x - x_0))^2 / b^2 - 1
    pub fn intersection_line_eq(&self, line: Line<T>) -> impl FnOnce(T) -> T
    where
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Sq<Output = NonNeg<T>>
            + Clone
            + One,
    {
        let x_0 = self.x.clone();
        let y_0 = self.y.clone();
        let a = self.a.clone();
        let b = self.b.clone();
        let r = self.r.clone();
        let i = self.i.clone();
        let k = line.k.clone();
        let d = line.d.clone();
        move |x| {
            ((r.clone() * (x.clone() - x_0.clone())
                - i.clone() * (k.clone() * x.clone() + d.clone() - y_0.clone()))
            .sq()
                / a.sq()
                + (r * (k * x.clone() + d - y_0) + i * (x - x_0)).sq() / b.sq())
            .into_inner()
                - T::one()
        }
    }

    /// returns discriminant of intersection equesion with `line`
    /// -(4 * (a^2 * (-k^2 * r^2 - 2 * i * k * r - i^2) + b^2 * (-i^2 * k^2 + 2 * i * k * r - r^2) + (r^4 + 2 * i^2 * r^2 + i^4) * (d^2 + 2 * d * (k * x_0 - y_0) + k^2 * x_0^2 - 2 * k * x_0 * y_0 + y_0^2))) / (a^2 * b^2)
    /// -(4 * (a_0^2 * (-k^2 * r_0^2 - 2 * i_0 * k * r_0 - i_0^2) + b_0^2 * (-i_0^2 * k^2 + 2 * i_0 * k * r_0 - r_0^2) + (r_0^4 + 2 * i_0^2 * r_0^2 + i_0^4) * (d^2 + 2 * d * (k * x_0 - y_0) + k^2 * x_0^2 - 2 * k * x_0 * y_0 + y_0^2))) / (a_0^2 * b_0^2)
    /// -(4 * (a_1^2 * (-k^2 * r_1^2 - 2 * i_1 * k * r_1 - i_1^2) + b_1^2 * (-i_1^2 * k^2 + 2 * i_1 * k * r_1 - r_1^2) + (r_1^4 + 2 * i_1^2 * r_1^2 + i_1^4) * (d^2 + 2 * d * (k * x_1 - y_1) + k^2 * x_1^2 - 2 * k * x_1 * y_1 + y_1^2))) / (a_1^2 * b_1^2)
    pub fn intersection_discriminant(&self, line: Line<T>) -> T
    where
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Sq<Output = NonNeg<T>>
            + Clone
            + One
            + Two
            + UnsignedContstant<4>,
    {
        let x_0 = self.x.clone();
        let y_0 = self.y.clone();
        let a = self.a.clone();
        let b = self.b.clone();
        let r = self.r.clone();
        let i = self.i.clone();
        let k = line.k.clone();
        let d = line.d.clone();

        uconst::<4, T>()
            * ((a.clone() * (r.clone() * k.clone() + i.clone())).sq()
                + (b.clone() * (i * k.clone() - r)).sq()
                - (d.clone() + k.clone() * x_0.clone()).sq()
                + T::two() * d * y_0.clone()
                + T::two() * k * x_0 * y_0.clone()
                - y_0.sq().into_inner())
            / (a * b).sq().into_inner()
    }

    /// returns `d` by given `k` where `y = kx + d` is a tangent to ellipse
    pub fn tangent_d(&self, k: T) -> (T, T)
    where
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Neg<Output = T>
            + Sq<Output = NonNeg<T>>
            + Clone,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        let x_0 = self.x.clone();
        let y_0 = self.y.clone();
        let a = self.a.clone();
        let b = self.b.clone();
        let r = self.r.clone();
        let i = self.i.clone();

        let discriminant =
            (a * (r.clone() * k.clone() + i.clone())).sq() + (b * (i * k.clone() - r)).sq();
        let base = -k * x_0 + y_0;

        (
            base.clone() + discriminant.clone().sqrt().into_inner(),
            base - discriminant.sqrt().into_inner(),
        )
    }

    /// intersection of this function with y = 0 is where common outer tangents are
    pub fn outer_tangents_fun(&self, rhs: &Self, k: T) -> (T, T)
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Sq<Output = NonNeg<T>> + Clone,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        let x_0 = self.x.clone();
        let y_0 = self.y.clone();
        let a_0 = self.a.clone();
        let b_0 = self.b.clone();
        let r_0 = self.r.clone();
        let i_0 = self.i.clone();

        let x_1 = rhs.x.clone();
        let y_1 = rhs.y.clone();
        let a_1 = rhs.a.clone();
        let b_1 = rhs.b.clone();
        let r_1 = rhs.r.clone();
        let i_1 = rhs.i.clone();

        let discriminant_0 = (a_0 * (r_0.clone() * k.clone() + i_0.clone())).sq()
            + (b_0 * (i_0 * k.clone() - r_0)).sq();
        let discriminant_1 = (a_1 * (r_1.clone() * k.clone() + i_1.clone())).sq()
            + (b_1 * (i_1 * k.clone() - r_1)).sq();

        let lhs = k * (x_1 - x_0) + y_0 - y_1;
        let rhs = discriminant_1.sqrt() - discriminant_0.sqrt();

        (lhs.clone() - rhs.clone(), lhs + rhs)
    }
}

#[inline(always)]
fn time_between_true_anomalies<T>(
    major_axis: T,
    eccentricity: NonNeg<T>,
    nu1: Angle<T>,
    nu2: Angle<T>,
    mu: T,
) -> Duration
where
    T: RemEuclid<Output = NonNeg<T>>
        + Cube<Output = T>
        + Atan<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Sin<Output = T>
        + Tan<Output = T>
        + AddAssign
        + IsNeg
        + One
        + Two
        + Pi
        + Clone,
    NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    Duration: FromSecs<T>,
{
    let e1 = true_anomaly_to_eccentric(eccentricity.clone(), nu1);
    let e2 = true_anomaly_to_eccentric(eccentricity.clone(), nu2);

    let m1 = eccentric_anomaly_to_mean(eccentricity.clone(), e1);
    let m2 = eccentric_anomaly_to_mean(eccentricity.clone(), e2);

    // Mean motion (n = sqrt(mu / a^3))
    let n = NonNeg::new(mu / major_axis.cube()).ok().unwrap().sqrt();

    let mut delta_m = m2 - m1;

    // Ensure the time is positive if traveling forward
    if delta_m.is_neg() {
        delta_m += T::two() * T::pi();
    }

    <Duration as FromSecs<T>>::from_secs(delta_m / n.into_inner())
}

#[inline(always)]
fn relative_time_between_true_anomalies<T>(
    major_axis: T,
    eccentricity: NonNeg<T>,
    nu1: Angle<T>,
    nu2: Angle<T>,
    mu: T,
) -> RelativeDuration
where
    T: RemEuclid<Output = NonNeg<T>>
        + Cube<Output = T>
        + Atan<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Sin<Output = T>
        + Tan<Output = T>
        + IsNeg
        + Clone
        + One
        + Two
        + Pi,
    NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    RelativeDuration: FromSecs<T>,
{
    let e1 = true_anomaly_to_eccentric(eccentricity.clone(), nu1);
    let e2 = true_anomaly_to_eccentric(eccentricity.clone(), nu2);

    let m1 = eccentric_anomaly_to_mean(eccentricity.clone(), e1);
    let m2 = eccentric_anomaly_to_mean(eccentricity, e2);

    // Mean motion (n = sqrt(mu / a^3))
    let n = NonNeg::new(mu / major_axis.cube()).ok().unwrap().sqrt();

    <RelativeDuration as FromSecs<T>>::from_secs((m2 - m1) / n.into_inner())
}

#[inline(always)]
fn true_anomaly_to_eccentric<T>(eccentricity: NonNeg<T>, anomaly: Angle<T>) -> T
where
    T: RemEuclid<Output = NonNeg<T>>
        + Atan<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Tan<Output = T>
        + IsNeg
        + Clone
        + One
        + Two
        + Pi,
    NonNeg<T>: Sqrt<Output = NonNeg<T>>,
{
    let factor = NonNeg::new(
        (T::one() - eccentricity.clone().into_inner()) / (T::one() + eccentricity.into_inner()),
    )
    .ok()
    .unwrap()
    .sqrt();
    T::two() * (factor.into_inner() * (anomaly.radians().into_inner() / T::two()).tan()).atan()
}

#[inline(always)]
fn eccentric_anomaly_to_mean<T>(eccentricity: NonNeg<T>, e_anom: T) -> T
where
    T: Sub<Output = T> + Mul<Output = T> + Sin<Output = T> + Clone,
{
    e_anom.clone() - eccentricity.into_inner() * e_anom.sin()
}

#[inline(always)]
fn absmax<T: Abs<Output = NonNeg<T>> + PartialOrd + IsNan + Clone>(a: T, b: T) -> T {
    if a.clone().is_nan() {
        return a;
    }

    if b.clone().is_nan() {
        return b;
    }

    if a.clone().abs() > b.clone().abs() {
        a
    } else {
        b
    }
}

#[inline(always)]
fn absmin<T: Abs<Output = NonNeg<T>> + PartialOrd + IsNan + Clone>(a: T, b: T) -> T {
    if a.clone().is_nan() {
        return a;
    }

    if b.clone().is_nan() {
        return b;
    }

    if a.clone().abs() < b.clone().abs() {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::Ellipse;

    #[allow(dead_code)]
    static ELLIPSE: Ellipse<f32> = Ellipse {
        x: -30.0,
        y: -100.0,
        a: 20.0,
        b: 80.0,
        r: 0.50000036,
        i: -0.8660252,
    };

    #[test]
    #[cfg(any(feature = "std", feature = "libm"))]
    fn rots() {
        use crate::{Angle, NonNeg};
        use approx::assert_abs_diff_eq;
        for i in 1..1000 {
            let anomaly = Angle::from_radians(i as f32 / 1000. * core::f32::consts::PI * 2.);

            let position = ELLIPSE.point_on_ellipse(anomaly);
            let anomaly2 = ELLIPSE.anomaly(position);

            assert_abs_diff_eq!(
                anomaly.radians(),
                anomaly2.radians(),
                epsilon = NonNeg::new(0.00001).unwrap()
            );
        }
    }
}
