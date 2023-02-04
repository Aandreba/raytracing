flat_mod! { vec2, vec3, vec4 }
flat_mod! { mat4 }
flat_mod! { euler, quat }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    Euler (EulerAngles),
    Quat (Quaternion)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Quaternion,
}

impl Transform {
    #[inline]
    pub fn translate (&mut self, offset: Vec3) {
        self.position += offset
    }
}

impl Transform {
    #[inline]
    pub fn apply (self, v: Vec3) -> Vec3 {
        unsafe {
            self.position + self.rotation.apply_unchecked(v).wide_mul(self.scale)
        }
    }
}

impl Default for Transform {
    #[inline]
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            scale: Vec3::splat(1.),
            rotation: Quaternion::new(1., 0., 0., 0.)
        }
    }
}

impl Rotation {
    #[inline]
    pub fn to_euler (self) -> Quaternion {
        todo!()
    }

    #[inline]
    pub fn to_quaternion (self) -> Quaternion {
        match self {
            Self::Quat(q) => q,
            Self::Euler(e) => Quaternion::from_euler(e)
        }
    }
}

impl Default for Rotation {
    #[inline]
    fn default() -> Self {
        Self::Quat(Quaternion::new(1., 0., 0., 0.))
    }
}

#[cfg(test)]
#[test]
fn test () {
    let t = Transform::default();
    let v = t.apply(Vec3::new(1., 2., 3.));
}