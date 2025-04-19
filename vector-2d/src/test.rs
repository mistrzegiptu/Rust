#[cfg(test)]
mod tests {
    use crate::Vec2D;
    use std::f32::consts::PI;
    use approx::assert_relative_eq;

    #[test]
    fn test_vec_addition() {
        let v1 = Vec2D { x: 1.0, y: 2.0 };
        let v2 = Vec2D { x: 3.0, y: 4.0 };
        let result = &v1 + &v2;
        assert_eq!(result, Vec2D { x: 4.0, y: 6.0 });
    }

    #[test]
    fn test_vec_subtraction() {
        let v1 = Vec2D { x: 5.0, y: 5.0 };
        let v2 = Vec2D { x: 3.0, y: 2.0 };
        let result = &v1 - &v2;
        assert_eq!(result, Vec2D { x: 2.0, y: 3.0 });
    }

    #[test]
    fn test_scalar_product() {
        let v1 = Vec2D { x: 1.0, y: 2.0 };
        let v2 = Vec2D { x: 3.0, y: 4.0 };
        assert_eq!(v1.scalar_product(&v2), 11.0);
    }

    #[test]
    fn test_mul_by_scalar() {
        let v = Vec2D { x: 2.0, y: 3.0 };
        let result = v.mul_by_scalar(2.0);
        assert_eq!(result, Vec2D { x: 4.0, y: 6.0 });
    }

    #[test]
    fn test_equality() {
        let v1 = Vec2D { x: 1.0, y: 2.0 };
        let v2 = Vec2D { x: 1.0, y: 2.0 };
        let v3 = Vec2D { x: 3.0, y: 4.0 };
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }

    #[test]
    fn test_display_formatting() {
        let v = Vec2D { x: 1.5, y: 2.5 };
        assert_eq!(format!("{}", v), "Vec2D[x=1.5,y=2.5]");
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec2D { x: 1.0, y: 2.0 };
        let v2 = Vec2D { x: 3.0, y: 4.0 };
        v1 = &v1 + &v2;
        assert_eq!(v1, Vec2D { x: 4.0, y: 6.0 });
    }

    #[test]
    fn test_edge_cases() {
        let zero = Vec2D { x: 0.0, y: 0.0 };
        let v = Vec2D { x: 1.0, y: 1.0 };

        assert_eq!(&zero + &v, v);
        assert_eq!(&v - &v, zero);
        assert_eq!(zero.scalar_product(&v), 0.0);
        assert_eq!(v.mul_by_scalar(0.0), zero);
    }

    #[test]
    fn test_vec_len_basic() {
        let v = Vec2D { x: 3.0, y: 4.0 };
        assert_eq!(v.vec_len(), 5.0);

        let v = Vec2D { x: 1.0, y: 1.0 };
        assert_relative_eq!(v.vec_len(), 2.0f32.sqrt());
    }

    #[test]
    fn test_vec_len_edge_cases() {
        let zero = Vec2D { x: 0.0, y: 0.0 };
        assert_eq!(zero.vec_len(), 0.0);

        let v_x = Vec2D { x: 5.0, y: 0.0 };
        assert_eq!(v_x.vec_len(), 5.0);

        let v_y = Vec2D { x: 0.0, y: -3.0 };
        assert_eq!(v_y.vec_len(), 3.0);
    }

    #[test]
    fn test_to_unit_vec_basic() {
        let v = Vec2D { x: 3.0, y: 4.0 };
        let unit = v.to_unit_vec();
        assert_relative_eq!(unit.x, 0.6);
        assert_relative_eq!(unit.y, 0.8);
        assert_relative_eq!(unit.vec_len(), 1.0);
    }

    #[test]
    fn test_to_unit_vec_direction_preserved() {
        let angles = [0.0, PI/4.0, PI/2.0, PI, 3.0*PI/2.0];
        for &angle in &angles {
            let x = angle.cos();
            let y = angle.sin();
            let v = Vec2D { x, y };
            let unit = v.to_unit_vec();

            assert_relative_eq!(unit.x, x.signum() * unit.x.abs());
            assert_relative_eq!(unit.y, y.signum() * unit.y.abs());
            assert_relative_eq!(unit.vec_len(), 1.0);
        }
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_to_unit_vec_zero_vector() {
        let zero = Vec2D { x: 0.0, y: 0.0 };
        let _ = zero.to_unit_vec();
    }

    #[test]
    fn test_to_unit_vec_properties() {
        let test_vectors = [
            Vec2D { x: 1.0, y: 2.0 },
            Vec2D { x: -3.0, y: 4.0 },
            Vec2D { x: 0.5, y: -0.5 },
            Vec2D { x: 100.0, y: 0.01 },
        ];

        for v in test_vectors {
            let unit = v.to_unit_vec();

            assert_relative_eq!(unit.vec_len(), 1.0, epsilon = 1e-6);

            let reconstructed = Vec2D {
                x: unit.x * v.vec_len(),
                y: unit.y * v.vec_len(),
            };
            assert_relative_eq!(reconstructed.x, v.x, epsilon = 1e-6);
            assert_relative_eq!(reconstructed.y, v.y, epsilon = 1e-6);
        }
    }
}