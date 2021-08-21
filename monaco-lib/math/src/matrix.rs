use macros::debug;

/// Matrix
pub struct Matrix
{
    pub data:Vec<f64>,
    pub rows:usize,
    pub cols:usize
}

/// Format the contents of the matrix into a string
/// 
/// # Arguments
/// 
/// * `matrix` - Matrix to be displayed
/// * `rows` - Matrix rows
/// * `cols` - Matrix columns
/// * `multi_line` - Add a '\n' character at the end of each line (instead of ',')
pub fn display_matrix(matrix:&Vec<f64>,rows:usize,cols:usize,multi_line:bool) -> String
{
    let mut data_str:String=String::new();
    for i in 0..rows
    {
        data_str.push_str("[");
        for j in 0..cols
        {
            data_str.push_str(format!("{}",get_item(&matrix,cols,i,j)).as_str());
            if j<cols-1
            {
                data_str.push_str(",");
            }
        }
        data_str.push_str("]");
        if multi_line
        {
            data_str.push_str("\n");
        }
        else
        {
            if i<rows-1
            {
                data_str.push_str(",");
            }
        }
    }
    let s=format!("{}{}{}","[",data_str,"]");
    return s;
}

pub fn get_item(matrix:&Vec<f64>,cols:usize,row:usize,col:usize) -> f64
{
    let idx=cols*row+col;
    return matrix[idx];
}

pub fn sub_matrix(matrix: &Vec<f64>, m:usize, start_row:usize, start_col:usize, rows:usize, cols:usize) -> Vec<f64>
{
    let mut subm:Vec<f64>=vec![0.0;rows*cols];
    for i in 0..rows
    {
        for j in 0..cols
        {
            subm[cols*i+j]=matrix[(m*(start_row+i))+(start_col+j)];
        }
    }
    return subm;
}

pub fn identity(n:usize) -> Vec<f64>
{
    let mut matrix:Vec<f64>=vec![0.0;n*n];
    for i in 0..n
    {
        matrix[i*n+i]=1.0;
    }
    return matrix;
}

pub fn identity_matrix(n:usize) -> Matrix
{
    let mut data:Vec<f64>=vec![0.0;n*n];
    for i in 0..n
    {
        data[i*n+i]=1.0;
    }
    let matrix=Matrix
    {
        data:data,
        rows:n,
        cols:n
    };
    return matrix;
}

pub fn horizontal_add(matrix1: &Vec<f64>,matrix2: &Vec<f64>, n:usize)-> Vec<f64>
{
    let mut added_matrix:Vec<f64>=vec![0.0;matrix1.len()+matrix2.len()];
    let m1:usize=matrix1.len()/n;
    let m2:usize=matrix2.len()/n;

    for i in 0..n
    {
        for j in 0..m1
        {
            added_matrix[i*(m1+m2)+j]=matrix1[i*m1+j];
        }
        for j in 0..m2
        {
            added_matrix[i*(m1+m2)+m1+j]=matrix2[i*m2+j];
        }
    }
    return added_matrix;
}

pub fn vertical_add(matrix1: &Vec<f64>,matrix2: &Vec<f64>)-> Vec<f64>
{
    let mut added_matrix:Vec<f64>=vec![0.0;matrix1.len()+matrix2.len()];

    for i in 0..matrix1.len()
    {
        added_matrix[i]=matrix1[i];
    }
    for i in 0..matrix2.len()
    {
        added_matrix[matrix1.len()+i]=matrix2[i];
    }
    return added_matrix;
}

/// Transpose a matrix
/// 
/// # Arguments
/// 
/// * `matrix` - Matrix to transpose
/// * `n` - Original number of rows 
/// * `m` - Original number of columns
pub fn transpose(matrix: &Vec<f64>, n:usize, m:usize) -> Vec<f64>
{
    let mut result = vec![0.0; matrix.len()];
    for i in 0..n
    {
        for j in 0..m
        {
            result[j*n+i]=matrix[i*m+j];
        }
    }
    return result;
}

/// Multiply two matrices
/// 
/// # Arguments
/// 
/// * `matrix_a` - First matrix
/// * `n_a` - Number of rows of first matrix
/// * `m_a` - Number of columns of first matrix
/// * `matrix_b` - Second matrix
/// * `n_b` - Number of rows of second matrix
/// * `m_b` - Number of columns of second matrix
pub fn multiply(matrix_a: &Vec<f64>, n_a:usize, m_a:usize, matrix_b: &Vec<f64>, n_b:usize, m_b:usize) -> Vec<f64>
{
    if m_a==n_b
    {
        let mut result = vec![0.0; n_a*m_b];
        for i in 0..n_a
        {
            for j in 0..m_b
            {
                for k in 0..m_a
                {
                    result[i*m_b+j]+=matrix_a[i*m_a+k]*matrix_b[k*m_b+j];
                }
            }
        }
        return result;
    }
    else
    {
        panic!("Matrices have incompatible dimensions");
    }
}

pub fn cholesky(matrix: &Vec<f64>) -> Vec<f64>
{
    let n=(matrix.len() as f64).sqrt() as usize;
    let mut result:Vec<f64>=vec![0.0; matrix.len()];
    for i in 0..n 
    {
        for j in 0..(i+1)
        {
            let mut sum = 0.0;
            for k in 0..j
            {
                sum += result[i * n + k] * result[j * n + k];
            }
            if i == j 
            {
                result[i * n + j] = f64::sqrt(matrix[i * n + i] - sum);
            }
            else
            {
                result[i * n + j] = (1.0 / result[j * n + j]) * (matrix[i * n + j] - sum);
            };
        }
    }
    //for i in 0..result.len()
    //{
    //    debug!(format!("cholesky [{}]: {}",i,result[i]));
    //}
    return result;
}

impl Matrix
{
    pub fn new(rows:usize,cols:usize,value:f64) -> Matrix
    {
        let data:Vec<f64>=vec![value;rows*cols];
        let result_matrix=Matrix{
            data:data,
            rows:rows,
            cols:cols
        };
        return result_matrix;   
    }
    fn set_item(&mut self,row:usize,col:usize,value:f64)
    {
        let idx=self.cols*row+col;
        self.data[idx]=value;
    }
    fn get_item(&self,row:usize,col:usize) -> f64
    {
        return get_item(&self.data,self.cols,row,col);
    }
    pub fn sub_matrix(&self, start_row:usize, start_col:usize, rows:usize, cols:usize) -> Matrix
    {
        let data=sub_matrix(&self.data, self.cols, start_row, start_col, rows, cols);
        let result_matrix=Matrix{
            data:data,
            rows:rows,
            cols:cols
        };
        return result_matrix;
    }
    pub fn horizontal_add(&self,matrix:&Matrix)-> Matrix
    {
        let data=horizontal_add(&self.data,&matrix.data, self.rows);
        let result_matrix=Matrix{
            data:data,
            rows:self.rows,
            cols:self.cols+matrix.cols
        };
        return result_matrix;
    }
    pub fn vertical_add(&self,matrix:&Matrix)-> Matrix
    {
        let data=vertical_add(&self.data,&matrix.data);
        let result_matrix=Matrix{
            data:data,
            rows:self.rows+matrix.rows,
            cols:self.cols
        };
        return result_matrix;
    }
    pub fn multiply(&self,matrix:&Matrix) -> Matrix
    {
        let result_data=multiply(&self.data, self.rows, self.cols, &matrix.data, matrix.rows, matrix.cols);
        let result_matrix=Matrix{
            data:result_data,
            rows:self.rows,
            cols:matrix.cols
        };
        return result_matrix;
    }
    pub fn transpose(&self) -> Matrix
    {
        let transposed_data=transpose(&self.data, self.rows, self.cols);
        let transposed_matrix=Matrix{
            data:transposed_data,
            rows:self.cols,
            cols:self.rows
        };
        return transposed_matrix;
    }


    pub fn print(&self) -> ()
    {
        for i in 0..self.rows
        {
            for j in 0..self.cols
            {
                println!("{}/{}: {}",i,j,self.data[i*self.cols+j]);
            }
        }
    }

    pub fn inverse(&self) -> Matrix
    {
        let id=identity_matrix(self.rows);
        let mut m2=self.horizontal_add(&id);
        
        let mut top_row =self.rows;
        let mut i=0;
        while i<top_row
        {
            let mut zero_row:bool = true;
            for j in 0..m2.cols
            {
                if m2.get_item(i, j) != 0.0 {zero_row = false;}
            }
            if zero_row
            {
                let x=m2.transpose().sub_matrix(0,0,i,m2.cols);
                let y=m2.transpose().sub_matrix(i + 1, 0, m2.rows - 1 - i, m2.cols);
                let z=m2.transpose().sub_matrix(i, 0, 1, m2.cols);

                m2=x.horizontal_add(&y).horizontal_add(&z).transpose();
                i=i-1;
                top_row=top_row-1;
            }   
            i=i+1;   
        }

        for k in 0..m2.rows
        {
            let mut found:bool = false;
                for j in 0..m2.cols
                {
                    let mut a:f64;
                    let mut row:Matrix = Matrix::new(0, 0, 0.0);
                    for i in k..m2.rows
                    {
                        if m2.get_item(i, j) != 0.0
                        {
                            if !found
                            {
                                found = true;
                                a = m2.get_item(i, j);
                                row = m2.sub_matrix(i, 0, 1, m2.cols);
                                for z in 0..row.cols
                                {
                                    row.set_item(0,z, row.get_item(0,z) / a);
                                }
                                let top:Matrix = m2.sub_matrix(0, 0, k, m2.cols);
                                let up:Matrix = m2.sub_matrix(k, 0, i - k, m2.cols);
                                let down:Matrix = m2.sub_matrix(i + 1, 0, m2.rows - i - 1, m2.cols);

                                m2 = top.vertical_add(&row);
                                m2 = m2.vertical_add(&up);
                                m2 = m2.vertical_add(&down);
                            }
                        }
                    }
                    let start:usize=0;
                    for i in start..m2.rows
                    {
                        if m2.get_item(i, j) != 0.0 && i != k && row.cols > 0
                        {
                            let lead:f64 = m2.get_item(i, j);
                            m2.set_item(i, j, 0.0);
                            for y in j + 1..m2.cols
                            {
                                let val=m2.get_item(i,y);
                                m2.set_item(i, y, val-(lead * row.get_item(0, y)));
                            }
                        }
                    }
                }
            }
            return m2.sub_matrix(0, m2.rows, m2.rows, m2.cols/2);
    }

    pub fn cholesky(&self) -> Matrix
    {
        let data=cholesky(&self.data);
        let result_matrix=Matrix
        {
            data:data,
            rows:self.rows,
            cols:self.cols
        };
        return result_matrix;
    }

}