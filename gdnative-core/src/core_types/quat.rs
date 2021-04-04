use super::{Basis, IsEqualApprox, Vector3, CMP_EPSILON};
use core::f32::consts::FRAC_PI_2;
use core::ops::Mul;
use glam::Mat3;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// Helper methods for `Quat`.
///
/// See the official [`Godot documentation`](https://docs.godotengine.org/en/stable/classes/class_quat.html).
impl Quat {
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// Constructs a quaternion defined by the given values.
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Constructs a quaternion from the given [Basis]
    #[inline]
    pub fn from_basis(basis: &Basis) -> Self {
        basis.to_quat()
    }

    /// Constructs a quaternion that will perform a rotation specified by Euler angles (in the YXZ
    /// convention: when decomposing, first Z, then X, and Y last), given in the vector format as
    /// (X angle, Y angle, Z angle).
    #[inline]
    pub fn from_euler(euler: Vector3) -> Self {
        Self::gd(glam::Quat::from_rotation_ypr(euler.y, euler.x, euler.z))
    }

    /// Constructs a quaternion that will rotate around the given axis by the specified angle. The
    /// axis must be a normalized vector.
    #[inline]
    pub fn from_axis_angle(axis: Vector3, angle: f32) -> Self {
        debug_assert!(axis.is_normalized(), "Axis is not normalized");
        Self::gd(glam::Quat::from_axis_angle(axis.glam().into(), angle))
    }

    /// Performs a cubic spherical interpolation between quaternions `pre_a`, this quaternion, `b`,
    /// and `post_b`, by the given amount `t`.
    #[inline]
    pub fn cubic_slerp(self, b: Self, pre_a: Self, post_b: Self, t: f32) -> Self {
        let t2 = (1.0 - t) * t * 2.0;
        let sp = self.slerp(b, t);
        let sq = pre_a.slerpni(post_b, t);
        sp.slerpni(sq, t2)
    }

    /// Returns the dot product of two quaternions.
    #[inline]
    pub fn dot(self, b: Self) -> f32 {
        self.glam().dot(b.glam())
    }

    /// Returns Euler angles (in the YXZ convention: when decomposing, first Z, then X, and Y last)
    /// corresponding to the rotation represented by the unit quaternion. Returned vector contains
    /// the rotation angles in the format (X angle, Y angle, Z angle).
    #[inline]
    pub fn get_euler(self) -> Vector3 {
        let basis = Mat3::from_quat(self.glam());
        let elm = basis.to_cols_array_2d();
        // Copied from Basis::get_euler_yxz
        let m12 = elm[1][2];
        if m12 < 1.0 - CMP_EPSILON as f32 {
            if m12 > CMP_EPSILON as f32 - 1.0 {
                #[allow(clippy::float_cmp)] // Godot also used direct comparison
                if elm[1][0] == 0.0
                    && elm[0][1] == 0.0
                    && elm[0][2] == 0.0
                    && elm[2][0] == 0.0
                    && elm[0][0] == 1.0
                {
                    Vector3::new((-m12).atan2(elm[1][1]), 0.0, 0.0)
                } else {
                    Vector3::new(
                        (-m12).asin(),
                        elm[0][2].atan2(elm[2][2]),
                        elm[1][0].atan2(elm[1][1]),
                    )
                }
            } else {
                Vector3::new(FRAC_PI_2, elm[0][1].atan2(elm[0][0]), 0.0)
            }
        } else {
            Vector3::new(-FRAC_PI_2, (-elm[0][1]).atan2(elm[0][0]), 0.0)
        }
    }

    /// Returns the inverse of the quaternion.
    #[inline]
    pub fn inverse(self) -> Self {
        Self::gd(self.glam().inverse())
    }

    /// Returns `true` if this quaternion and `quat` are approximately equal, by running
    /// `is_equal_approx` on each component
    #[inline]
    pub fn is_equal_approx(self, to: Self) -> bool {
        self.x.is_equal_approx(to.x)
            && self.y.is_equal_approx(to.y)
            && self.z.is_equal_approx(to.z)
            && self.w.is_equal_approx(to.w)
    }

    /// Returns whether the quaternion is normalized or not.
    #[inline]
    pub fn is_normalized(self) -> bool {
        self.glam().is_normalized()
    }

    /// Returns the length of the quaternion.
    #[inline]
    pub fn length(self) -> f32 {
        self.glam().length()
    }

    /// Returns the length of the quaternion, squared.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.glam().length_squared()
    }

    /// Returns a copy of the quaternion, normalized to unit length.
    #[inline]
    pub fn normalized(self) -> Self {
        Self::gd(self.glam().normalize())
    }

    /// Sets the quaternion to a rotation which rotates around axis by the specified angle, in
    /// radians. The axis must be a normalized vector.
    #[inline]
    pub fn set_axis_angle(&mut self, axis: Vector3, angle: f32) {
        *self = Self::from_axis_angle(axis, angle)
    }

    /// Sets the quaternion to a rotation specified by Euler angles (in the YXZ convention: when
    /// decomposing, first Z, then X, and Y last), given in the vector format as (X angle, Y angle,
    /// Z angle).
    #[inline]
    pub fn set_euler(&mut self, euler: Vector3) {
        *self = Self::from_euler(euler);
    }

    /// Returns the result of the spherical linear interpolation between this quaternion and to by
    /// amount weight.
    ///
    /// **Note:** Both quaternions must be normalized.
    #[inline]
    pub fn slerp(self, b: Self, t: f32) -> Self {
        debug_assert!(self.is_normalized(), "Quaternion `self` is not normalized");
        debug_assert!(b.is_normalized(), "Quaternion `b` is not normalized");
        Self::gd(self.glam().slerp(b.glam(), t))
    }

    /// Returns the result of the spherical linear interpolation between this quaternion and `t` by
    /// amount `t`, but without checking if the rotation path is not bigger than 90 degrees.
    #[inline]
    pub fn slerpni(self, b: Self, t: f32) -> Self {
        debug_assert!(self.is_normalized(), "Quaternion `self` is not normalized");
        debug_assert!(b.is_normalized(), "Quaternion `b` is not normalized");
        let dot = self.dot(b);
        if dot.abs() > 0.9999 {
            self
        } else {
            let theta = dot.acos();
            let sin_t = 1.0 / theta.sin();
            let new_f = (t * theta).sin() * sin_t;
            let inv_f = ((1.0 - t) * theta).sin() * sin_t;
            Self::new(
                inv_f * self.x + new_f * b.x,
                inv_f * self.y + new_f * b.y,
                inv_f * self.z + new_f * b.z,
                inv_f * self.w + new_f * b.w,
            )
        }
    }

    /// Returns a vector transformed (multiplied) by this quaternion.
    #[inline]
    pub fn xform(self, v: Vector3) -> Vector3 {
        self * v
    }

    #[inline]
    fn gd(quat: glam::Quat) -> Self {
        Self::new(quat.x, quat.y, quat.z, quat.w)
    }

    #[inline]
    fn glam(self) -> glam::Quat {
        glam::Quat::from_xyzw(self.x, self.y, self.z, self.w)
    }
}

impl Mul<Vector3> for Quat {
    type Output = Vector3;

    #[inline]
    fn mul(self, with: Vector3) -> Vector3 {
        debug_assert!(self.is_normalized(), "Quaternion is not normalized");
        Vector3::gd(self.glam() * with.glam())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_euler() {
        let euler = Vector3::new(0.25, 5.24, 3.0);
        let expect = Quat::new(0.485489, 0.142796, -0.862501, 0.001113);
        assert!(Quat::from_euler(euler).is_equal_approx(expect));
    }

    #[test]
    fn to_basis() {
        let quat = Quat::new(0.485489, 0.142796, -0.862501, 0.001113);
        let expect = Basis::from_elements([
            Vector3::new(-0.528598, 0.140572, -0.837152),
            Vector3::new(0.136732, -0.959216, -0.247404),
            Vector3::new(-0.837788, -0.245243, 0.487819),
        ]);
        let basis = Mat3::from_quat(quat.glam()).to_cols_array_2d();
        let basis = [
            Vector3::new(basis[0][0], basis[1][0], basis[2][0]),
            Vector3::new(basis[0][1], basis[1][1], basis[2][1]),
            Vector3::new(basis[0][2], basis[1][2], basis[2][2]),
        ];
        let basis = Basis::from_elements(basis);
        assert!(basis.is_equal_approx(&expect));
    }

    #[test]
    fn get_euler() {
        let quat = Quat::new(0.485489, 0.142796, -0.862501, 0.001113);
        let expect = Vector3::new(0.25, -1.043185, 3.00001);
        dbg!(quat.get_euler());
        assert!(quat.get_euler().is_equal_approx(expect));
    }

    #[test]
    fn mul() {
        let quat = Quat::new(0.485489, 0.142796, -0.862501, 0.001113);
        let vec = Vector3::new(2.2, 0.8, 1.65);
        let expect = Vector3::new(-2.43176, -0.874777, -1.234427);
        assert!(expect.is_equal_approx(quat * vec));
    }
}
