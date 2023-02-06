flat_mod! { vec2, vec3, vec4 }
flat_mod! { mat4 }
flat_mod! { euler, quat }

/// Describes a tranformation in 3D-space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Versor,
    pub scale: Vec3,
}

impl Transform {
    #[inline]
    pub fn new(position: Vec3, rotation: Versor, scale: Vec3) -> Self {
        return Self {
            position,
            rotation,
            scale,
        };
    }

    #[inline]
    pub fn translate(&mut self, offset: Vec3) {
        self.position += offset
    }

    #[inline]
    pub fn rotate(&mut self, rot: Versor) {
        self.rotation *= rot
    }

    #[inline]
    pub fn scale(&mut self, scale: Vec3) {
        self.scale = self.scale.wide_mul(scale)
    }
}

impl Transform {
    #[inline]
    pub fn apply(self, v: Vec3) -> Vec3 {
        self.position + self.rotation.apply(v).wide_mul(self.scale)
    }
}

impl Default for Transform {
    #[inline]
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), Vec3::splat(1.))
    }
}

#[cfg(test)]
#[test]
fn test() {
    let t = Transform::new(
        Vec3::new(1., 1., 0.),
        EulerAngles::from_radians(1., 2., 3.).to_versor(),
        Vec3::splat(0.5),
    );

    println!("{t:?}");
    let v = t.apply(Vec3::new(1., 2., 3.));
    println!("{:?} --> {v:?}", Vec3::new(1., 2., 3.));
}
