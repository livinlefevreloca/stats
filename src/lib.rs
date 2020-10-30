#![allow(non_snake_case, non_camel_case_types)]
use std::fmt::{self, Display, Formatter};
use std::ops::{Mul, AddAssign};

#[derive(Debug, Clone, PartialEq)]


pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}



impl<T> Display for Matrix<T>
where
    T: fmt::Display,
{

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut res = String::new();
        for elem in &self.data {
            let base: Vec<String> = elem.iter().map(|x| x.to_string()).collect();
            res += &(base.join(",") + &"\n       ".to_string());
        }
        res.truncate(res.len() - 8);
        write!(f, "Matrix({})", res)
    }
}

impl<T> Matrix<T>
where
    T: Mul<T, Output = T> + AddAssign<T> + Copy + Default 
    {

    pub fn dot(&self, other: &Self) -> Self {
        let mut data = Vec::new();
        for i in 0..self.data.len() {
            let mut tmp = Vec::new();
            for j in 0..other.data[0].len() {
                let mut accum = T::default();
                for k in 0..other.data.len() {
                    accum += self.data[i][k] * other.data[k][j];
                }
                tmp.push(accum);
            }
            data.push(tmp);
        }
        Matrix { data }
    }
    
    pub fn transpose(&self) -> Self {
        let mut data = Vec::new();
        for i in 0..self.data[0].len() {
            let mut tmp = Vec::new();
            for j in 0..self.data.len() {
                tmp.push(self.data[j][i]);
            }
            data.push(tmp);
        }
        Matrix { data }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_dot_two_float() {
        
        let data: Vec<Vec<f64>> = vec![vec![2., 2.], vec![4., 4.]];
        let A = Matrix { data };
        let B = A.clone();
        
        let dotted = A.dot(&B);
        
        let result = Matrix { data: vec![vec![12., 12.], vec![24., 24.]] };
        assert_eq!(dotted, result);
    }
    
    #[test]
    fn two_dot_two_i32() {
        
        let data: Vec<Vec<i32>> = vec![vec![2, 2], vec![4, 4]];
        let A = Matrix { data };
        let B = A.clone();
        
        let dotted = A.dot(&B);
        
        let result = Matrix { data: vec![vec![12, 12], vec![24, 24]] };
        assert_eq!(dotted, result);
    }

    #[test]
    fn three_dot_three_float() {
        let data_3 = vec![vec![2., 2., 2.], vec![4., 4., 4.], vec![6., 6., 6.]];
        let C = Matrix { data: data_3 };
        let D = C.clone();

        let dotted = C.dot(&D);
        
        let result = Matrix { data: vec![vec![24., 24., 24.], vec![48., 48., 48.], vec![72., 72., 72.]] };
        assert_eq!(dotted, result);
    }

    #[test]
    fn transpose_two_by_three() {
        let data = vec![vec![1, 2], vec![3, 4], vec![5,6]];
        let A = Matrix { data };

        let A_T = A.transpose();
        let result = Matrix { data: vec![vec![1, 3, 5], vec![2, 4, 6]]};
        assert_eq!(A_T, result);

    } 
}
