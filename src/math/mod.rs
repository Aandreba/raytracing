flat_mod! { vec2, vec3, vec4 }
flat_mod! { mat4 }
flat_mod! { quat }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Quaternion,
}

impl Transform {
    #[inline]
    pub fn apply (&self, v: Vec3) -> Vec3 {
        self.ro (v + self.position)
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