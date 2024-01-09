use super::math::Vec3;

#[derive(Clone)]
pub enum Material {
    Lambertian(Vec3),    // diffuse material, Vec3 stands for color
    Metallic(Vec3, f32), // metallic material, Vec3 stands for color, f32 stands for fuzziness
    Dielectric(f32),     // dielectric material, f32 stands for refraction index
    Emissive(Vec3),      // emissive material, Vec3 stands for color
}
