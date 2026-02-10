use crate::math::{Complex, Matrix, One, Point, Vector, Zero};
use core::ops::{Add, Mul, Neg, Sub};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Camera<T> {
    translation: Matrix<T>,
    scale: Matrix<T>,
    rotation: Matrix<T>,
}

impl<T> Default for Camera<T>
where
    T: Zero + One,
{
    fn default() -> Self {
        Self {
            translation: Matrix::identity(),
            scale: Matrix::identity(),
            rotation: Matrix::identity(),
        }
    }
}

impl<T> Camera<T> {
    pub fn set_translation(&mut self, translation: Point<T>)
    where
        T: Zero + One + Clone + Sub<Output = T>,
    {
        self.translation = Matrix::translate(translation - Point::origin());
    }

    pub fn set_scale(&mut self, s: T)
    where
        T: Zero + One + Clone,
    {
        self.scale = Matrix::scale(s.clone(), s);
    }

    pub fn set_rotation(&mut self, rotor: Complex<T>)
    where
        T: Zero + One + Clone + Neg<Output = T>,
    {
        self.rotation = Matrix::rotate(rotor);
    }

    pub fn add_translation(&mut self, vec: Vector<T>)
    where
        T: Zero + One + Clone + Add<Output = T>,
    {
        self.translation = Matrix::translate(self.translation().translation() + vec);
    }

    /**
     * @brief concat_scale_centered
     * @param scale_division - always > 0 (if > 1 - scale in, else if < 1 - scale out, else no scale)
     * @param center - center of scaling.
     * @param prev_center - pass the same value as a center if you want only scale, different if translate is intended
     * @return absolute value of scale after concatenation
     */
    pub fn concat_scale_centered(
        &mut self,
        scale_division: T,
        center: Point<T>,
        prev_center: Point<T>,
    ) where
        T: Clone + Zero + One + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
    {
        concat_scale_centered(
            &mut self.scale,
            &mut self.translation,
            scale_division,
            center,
            prev_center,
        )
    }

    pub fn translation(&self) -> &Matrix<T> {
        &self.translation
    }

    pub fn scale(&self) -> &Matrix<T> {
        &self.scale
    }

    pub fn rotation(&self) -> &Matrix<T> {
        &self.rotation
    }

    pub fn transformation(&self) -> Matrix<T>
    where
        T: Clone + Mul<Output = T> + Add<Output = T>,
    {
        &self.translation * &self.scale * &self.rotation
    }
}

fn concat_scale_centered<T>(
    scale_output: &mut Matrix<T>,
    translation_output: &mut Matrix<T>,
    scale_division: T,
    center: Point<T>,
    prev_center: Point<T>,
) where
    T: Clone + Zero + One + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
{
    let filter_accepts_scale =
        |m: &Matrix<T>| Matrix::scale(m.scale_x().clone(), m.scale_y().clone());
    let filter_accepts_translation = |m: &Matrix<T>| Matrix::translate(m.translation());

    let scale_division_matrix: Matrix<T> = Matrix::scale(scale_division.clone(), scale_division);
    let translation = Matrix::translate(center.clone() - Point::origin());
    let inv_translation = Matrix::translate(Point::origin() - prev_center);

    let output = &translation
        * &scale_division_matrix
        * &inv_translation
        * &*translation_output
        * &*scale_output;

    *scale_output = filter_accepts_scale(&output);
    *translation_output = filter_accepts_translation(&output);
}
