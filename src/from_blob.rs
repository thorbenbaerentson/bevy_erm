use bevy::prelude::*;

pub trait FromBlob {
    fn from_blob(value: &[u8]) -> Self;
}

pub trait IntoBlob {
    fn into_blob(self) -> Vec<u8>;
}

// Vec
impl IntoBlob for Vec2 {
    fn into_blob(self) -> Vec<u8> {
        [f32::to_le_bytes(self.x), f32::to_le_bytes(self.y)].concat()
    }
}

impl FromBlob for Vec2 {
    fn from_blob(value: &[u8]) -> Self {
        let x = f32::from_le_bytes(value[0..4].try_into().expect("Not enough bites"));
        let y = f32::from_le_bytes(value[4..8].try_into().expect("Not enough bites"));

        Vec2::new(x, y)
    }
}

impl IntoBlob for Vec3 {
    fn into_blob(self) -> Vec<u8> {
        [
            f32::to_le_bytes(self.x),
            f32::to_le_bytes(self.y),
            f32::to_le_bytes(self.z),
        ]
        .concat()
    }
}

impl FromBlob for Vec3 {
    fn from_blob(value: &[u8]) -> Self {
        let x = f32::from_le_bytes(value[0..4].try_into().expect("Not enough bites"));
        let y = f32::from_le_bytes(value[4..8].try_into().expect("Not enough bites"));
        let z = f32::from_le_bytes(value[8..12].try_into().expect("Not enough bites"));

        Vec3::new(x, y, z)
    }
}

impl IntoBlob for Vec4 {
    fn into_blob(self) -> Vec<u8> {
        [
            f32::to_le_bytes(self.x),
            f32::to_le_bytes(self.y),
            f32::to_le_bytes(self.z),
            f32::to_le_bytes(self.w),
        ]
        .concat()
    }
}

impl FromBlob for Vec4 {
    fn from_blob(value: &[u8]) -> Self {
        let x = f32::from_le_bytes(value[0..4].try_into().expect("Not enough bites"));
        let y = f32::from_le_bytes(value[4..8].try_into().expect("Not enough bites"));
        let z = f32::from_le_bytes(value[8..12].try_into().expect("Not enough bites"));
        let w = f32::from_le_bytes(value[12..16].try_into().expect("Not enough bites"));

        Vec4::new(x, y, z, w)
    }
}

// Quat
impl IntoBlob for Quat {
    fn into_blob(self) -> Vec<u8> {
        [
            f32::to_le_bytes(self.x),
            f32::to_le_bytes(self.y),
            f32::to_le_bytes(self.z),
            f32::to_le_bytes(self.w),
        ]
        .concat()
    }
}

impl FromBlob for Quat {
    fn from_blob(value: &[u8]) -> Self {
        let x = f32::from_le_bytes(value[0..4].try_into().expect("Not enough bites"));
        let y = f32::from_le_bytes(value[4..8].try_into().expect("Not enough bites"));
        let z = f32::from_le_bytes(value[8..12].try_into().expect("Not enough bites"));
        let w = f32::from_le_bytes(value[12..16].try_into().expect("Not enough bites"));

        Quat::from_xyzw(x, y, z, w)
    }
}

// IVec
impl IntoBlob for IVec2 {
    fn into_blob(self) -> Vec<u8> {
        [i32::to_le_bytes(self.x), i32::to_le_bytes(self.y)].concat()
    }
}

impl FromBlob for IVec2 {
    fn from_blob(value: &[u8]) -> Self {
        let x = i32::from_le_bytes(value[0..4].try_into().expect("Not enough bites"));
        let y = i32::from_le_bytes(value[4..8].try_into().expect("Not enough bites"));

        IVec2::new(x, y)
    }
}

impl IntoBlob for IVec3 {
    fn into_blob(self) -> Vec<u8> {
        [
            i32::to_le_bytes(self.x),
            i32::to_le_bytes(self.y),
            i32::to_le_bytes(self.z),
        ]
        .concat()
    }
}

impl FromBlob for IVec3 {
    fn from_blob(value: &[u8]) -> Self {
        let x = i32::from_le_bytes(value[0..4].try_into().expect("Not enough bites"));
        let y = i32::from_le_bytes(value[4..8].try_into().expect("Not enough bites"));
        let z = i32::from_le_bytes(value[8..12].try_into().expect("Not enough bites"));

        IVec3::new(x, y, z)
    }
}

impl IntoBlob for IVec4 {
    fn into_blob(self) -> Vec<u8> {
        [
            i32::to_le_bytes(self.x),
            i32::to_le_bytes(self.y),
            i32::to_le_bytes(self.z),
            i32::to_le_bytes(self.w),
        ]
        .concat()
    }
}

impl FromBlob for IVec4 {
    fn from_blob(value: &[u8]) -> Self {
        let x = i32::from_le_bytes(value[0..4].try_into().expect("Not enough bites"));
        let y = i32::from_le_bytes(value[4..8].try_into().expect("Not enough bites"));
        let z = i32::from_le_bytes(value[8..12].try_into().expect("Not enough bites"));
        let w = i32::from_le_bytes(value[12..16].try_into().expect("Not enough bites"));

        IVec4::new(x, y, z, w)
    }
}

// UVec
impl IntoBlob for UVec2 {
    fn into_blob(self) -> Vec<u8> {
        [u32::to_le_bytes(self.x), u32::to_le_bytes(self.y)].concat()
    }
}

impl FromBlob for UVec2 {
    fn from_blob(value: &[u8]) -> Self {
        let x = u32::from_le_bytes(value[0..4].try_into().expect("Not enough bites"));
        let y = u32::from_le_bytes(value[4..8].try_into().expect("Not enough bites"));

        UVec2::new(x, y)
    }
}

impl IntoBlob for UVec3 {
    fn into_blob(self) -> Vec<u8> {
        [
            u32::to_le_bytes(self.x),
            u32::to_le_bytes(self.y),
            u32::to_le_bytes(self.z),
        ]
        .concat()
    }
}

impl FromBlob for UVec3 {
    fn from_blob(value: &[u8]) -> Self {
        let x = u32::from_le_bytes(value[0..4].try_into().expect("Not enough bites"));
        let y = u32::from_le_bytes(value[4..8].try_into().expect("Not enough bites"));
        let z = u32::from_le_bytes(value[8..12].try_into().expect("Not enough bites"));

        UVec3::new(x, y, z)
    }
}

impl IntoBlob for UVec4 {
    fn into_blob(self) -> Vec<u8> {
        [
            u32::to_le_bytes(self.x),
            u32::to_le_bytes(self.y),
            u32::to_le_bytes(self.z),
            u32::to_le_bytes(self.w),
        ]
        .concat()
    }
}

impl FromBlob for UVec4 {
    fn from_blob(value: &[u8]) -> Self {
        let x = u32::from_le_bytes(value[0..4].try_into().expect("Not enough bites"));
        let y = u32::from_le_bytes(value[4..8].try_into().expect("Not enough bites"));
        let z = u32::from_le_bytes(value[8..12].try_into().expect("Not enough bites"));
        let w = u32::from_le_bytes(value[12..16].try_into().expect("Not enough bites"));

        UVec4::new(x, y, z, w)
    }
}

impl IntoBlob for Srgba {
    fn into_blob(self) -> Vec<u8> {
        [
            f32::to_le_bytes(self.red),
            f32::to_le_bytes(self.green),
            f32::to_le_bytes(self.blue),
            f32::to_le_bytes(self.alpha),
        ]
        .concat()
    }
}

impl FromBlob for Srgba {
    fn from_blob(value: &[u8]) -> Self {
        let red = f32::from_le_bytes(value[0..4].try_into().expect("Not enough bites"));
        let green = f32::from_le_bytes(value[4..8].try_into().expect("Not enough bites"));
        let blue = f32::from_le_bytes(value[8..12].try_into().expect("Not enough bites"));
        let alpha = f32::from_le_bytes(value[12..16].try_into().expect("Not enough bites"));

        Srgba::new(red, green, blue, alpha)
    }
}

impl IntoBlob for Rect {
    fn into_blob(self) -> Vec<u8> {
        [
            f32::to_le_bytes(self.min.x),
            f32::to_le_bytes(self.min.y),
            f32::to_le_bytes(self.max.x),
            f32::to_le_bytes(self.max.y),
        ]
        .concat()
    }
}

impl FromBlob for Rect {
    fn from_blob(value: &[u8]) -> Self {
        let x1 = f32::from_le_bytes(value[0..4].try_into().expect("Not enough bites"));
        let y1 = f32::from_le_bytes(value[4..8].try_into().expect("Not enough bites"));
        let x2 = f32::from_le_bytes(value[8..12].try_into().expect("Not enough bites"));
        let y2 = f32::from_le_bytes(value[12..16].try_into().expect("Not enough bites"));

        Rect::new(x1, y1, x2, y2)
    }
}

impl IntoBlob for IRect {
    fn into_blob(self) -> Vec<u8> {
        [
            i32::to_le_bytes(self.min.x),
            i32::to_le_bytes(self.min.y),
            i32::to_le_bytes(self.max.x),
            i32::to_le_bytes(self.max.y),
        ]
        .concat()
    }
}

impl FromBlob for IRect {
    fn from_blob(value: &[u8]) -> Self {
        let x1 = i32::from_le_bytes(value[0..4].try_into().expect("Not enough bites"));
        let y1 = i32::from_le_bytes(value[4..8].try_into().expect("Not enough bites"));
        let x2 = i32::from_le_bytes(value[8..12].try_into().expect("Not enough bites"));
        let y2 = i32::from_le_bytes(value[12..16].try_into().expect("Not enough bites"));

        IRect::new(x1, y1, x2, y2)
    }
}

#[cfg(test)]
mod tests {
    use super::{FromBlob, IntoBlob};
    use bevy::prelude::*;

    #[test]
    fn test_vec2() {
        for x in 1..10 {
            for y in 1..10 {
                let subject = Vec2::new(x as f32, y as f32);
                let blob = subject.into_blob();
                let test = Vec2::from_blob(&blob);

                assert_eq!(subject, test);
            }
        }
    }

    #[test]
    fn test_vec3() {
        for x in 1..10 {
            for y in 1..10 {
                for z in 1..10 {
                    let subject = Vec3::new(x as f32, y as f32, z as f32);
                    let blob = subject.into_blob();
                    let test = Vec3::from_blob(&blob);

                    assert_eq!(subject, test);
                }
            }
        }
    }

    #[test]
    fn test_vec4() {
        for x in 1..10 {
            for y in 1..10 {
                for z in 1..10 {
                    for w in 1..10 {
                        let subject = Vec4::new(x as f32, y as f32, z as f32, w as f32);
                        let blob = subject.into_blob();
                        let test = Vec4::from_blob(&blob);

                        assert_eq!(subject, test);
                    }
                }
            }
        }
    }

    #[test]
    fn test_quat() {
        for x in 1..10 {
            for y in 1..10 {
                for z in 1..10 {
                    for w in 1..10 {
                        let subject = Quat::from_xyzw(x as f32, y as f32, z as f32, w as f32);
                        let blob = subject.into_blob();
                        let test = Quat::from_blob(&blob);

                        assert_eq!(subject, test);
                    }
                }
            }
        }
    }

    #[test]
    fn test_ivec2() {
        for x in 1..10 {
            for y in 1..10 {
                let subject = IVec2::new(x, y);
                let blob = subject.into_blob();
                let test = IVec2::from_blob(&blob);

                assert_eq!(subject, test);
            }
        }
    }

    #[test]
    fn test_ivec3() {
        for x in 1..10 {
            for y in 1..10 {
                for z in 1..10 {
                    let subject = IVec3::new(x, y, z);
                    let blob = subject.into_blob();
                    let test = IVec3::from_blob(&blob);

                    assert_eq!(subject, test);
                }
            }
        }
    }

    #[test]
    fn test_ivec4() {
        for x in 1..10 {
            for y in 1..10 {
                for z in 1..10 {
                    for w in 1..10 {
                        let subject = IVec4::new(x, y, z, w);
                        let blob = subject.into_blob();
                        let test = IVec4::from_blob(&blob);

                        assert_eq!(subject, test);
                    }
                }
            }
        }
    }

    #[test]
    fn test_uvec2() {
        for x in 1..10 {
            for y in 1..10 {
                let subject = UVec2::new(x, y);
                let blob = subject.into_blob();
                let test = UVec2::from_blob(&blob);

                assert_eq!(subject, test);
            }
        }
    }

    #[test]
    fn test_uvec3() {
        for x in 1..10 {
            for y in 1..10 {
                for z in 1..10 {
                    let subject = UVec3::new(x, y, z);
                    let blob = subject.into_blob();
                    let test = UVec3::from_blob(&blob);

                    assert_eq!(subject, test);
                }
            }
        }
    }

    #[test]
    fn test_uvec4() {
        for x in 1..10 {
            for y in 1..10 {
                for z in 1..10 {
                    for w in 1..10 {
                        let subject = UVec4::new(x, y, z, w);
                        let blob = subject.into_blob();
                        let test = UVec4::from_blob(&blob);

                        assert_eq!(subject, test);
                    }
                }
            }
        }
    }

    #[test]
    fn test_srgb() {
        for x in 1..10 {
            for y in 1..10 {
                for z in 1..10 {
                    for w in 1..10 {
                        let subject = Srgba::from_vec4(Vec4::new(x as f32, y as f32, z as f32, w as f32));
                        let blob = subject.into_blob();
                        let test = Srgba::from_blob(&blob);

                        assert_eq!(subject, test);
                    }
                }
            }
        }
    }

    #[test]
    fn test_rect() {
        for x in 1..10 {
            for y in 1..10 {
                for z in 1..10 {
                    for w in 1..10 {
                        let subject = Rect::new(x as f32, y as f32, z as f32, w as f32);
                        let blob = subject.into_blob();
                        let test = Rect::from_blob(&blob);

                        assert_eq!(subject, test);
                    }
                }
            }
        }
    }

    #[test]
    fn test_irect() {
        for x in 1..10 {
            for y in 1..10 {
                for z in 1..10 {
                    for w in 1..10 {
                        let subject = IRect::new(x, y, z, w);
                        let blob = subject.into_blob();
                        let test = IRect::from_blob(&blob);

                        assert_eq!(subject, test);
                    }
                }
            }
        }
    }
}
