fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3]
{
    let mut result: [[i32; 3]; 3] = [[0; 3]; 3];    // Создаём новую матрицу 3×3, заполненную нулями. mut позволяет изменять элементы матрицы
    for i in 0..3   // Проходим по всем строкам исходной матрицы
    {
        for j in 0..3   // Проходим по всем столбцам исходной матрицы
        {
            result[j][i] = matrix[i][j];    // Транспонирование: элемент [i][j] переходит в [j][i]
        }
    }
    result  // Возвращаем транспонированную матрицу
}

#[test]
fn test_transpose()
{
    let matrix: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    let transposed: [[i32; 3]; 3] = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [1, 4, 7],
            [2, 5, 8],
            [3, 6, 9],
        ]
    );
}

fn main()
{
    let matrix: [[i32; 3]; 3] = [   // массив matrix содержит 3 элемента, которые, в свою очередь, также являются массивами. Причем каждый такой вложенный массив имеет три элемента типа i32
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    println!("исходная матрица:");
    for numbers in matrix
    {
        println!("{:?}", numbers); 
    }

    let transposed: [[i32; 3]; 3] = transpose(matrix);

    println!("транспонированная матрица:");
    for numbers in transposed
    {
        println!("{:?}", numbers); 
    }
    
}