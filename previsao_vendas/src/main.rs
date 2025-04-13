struct RegistroVenda {

    mes: f64,

    valor: f64,

}

fn calcular_media (valores: &[f64]) -> f64 {

    let soma: f64 = valores.iter().sum::<f64>();

    let media: f64 = soma / valores.len() as f64;

    media

}

fn main() {

    let teste_venda = RegistroVenda {

        mes: 1.0,

        valor: 1000.99,

    };

    let valores = vec![1.0, 2.0, 3.0];

    let media =  calcular_media(&valores);

    println!(A média de vendas é: {}", media);

    println!("Mês: {} Vendas: {}", (teste_venda.mes, teste_venda.valor);



extern crate ndarray;
extern crate ndarray_linalg;

use ndarray::prelude::*;
use ndarray_linalg::solve::Inverse;

fn linear_regression(x: &Array2<f64>, y: &Array1<f64>) -> Result<Array1<f64>, &'static str> {
    if x.nrows() != y.len() {
        return Err("O número de linhas em x deve ser igual ao número de elementos em y.");
    }

    // Adiciona uma coluna de 1s para o termo de interceptação
    let ones = Array::ones((x.nrows(), 1));
    let x = concatenate![Axis(1), ones, x];

    // Calcula os coeficientes usando a fórmula (X^T * X)^-1 * X^T * y
    let xt = x.t();
    let xtx = xt.dot(&x);
    let xtx_inv = xtx.inv().map_err(|_| "Não foi possível calcular a inversa de X^T * X")?;
    let xty = xt.dot(y);
    let coefficients = xtx_inv.dot(&xty);

    Ok(coefficients)
}

fn main() {
    // Exemplo de uso
    let x = array![[1.0, 2.0], [2.0, 3.0], [3.0, 4.0], [4.0, 5.0]];
    let y = array![2.0, 3.0, 4.0, 5.0];

    match linear_regression(&x, &y) {
        Ok(coefficients) => println!("Coeficientes: {:?}", coefficients),
        Err(e) => println!("Erro: {}", e),
    }
}


fn main() {
    let true_labels = vec![1, 0, 1, 1, 0, 1, 0, 0, 1, 0];
    let predictions = vec![1, 0, 1, 0, 0, 1, 0, 1, 1, 0];

    let (precision, recall, f1_score) = calculate_metrics(&true_labels, &predictions);

    println!("Precisão: {:.2}", precision);
    println!("Recall: {:.2}", recall);
    println!("F1-Score: {:.2}", f1_score);
}

fn calculate_metrics(true_labels: &[i32], predictions: &[i32]) -> (f64, f64, f64) {
    let mut true_positive = 0;
    let mut false_positive = 0;
    let mut false_negative = 0;

    for (true_label, prediction) in true_labels.iter().zip(predictions.iter()) {
        if *true_label == 1 && *prediction == 1 {
            true_positive += 1;
        } else if *true_label == 0 && *prediction == 1 {
            false_positive += 1;
        } else if *true_label == 1 && *prediction == 0 {
            false_negative += 1;
        }
    }

    let precision = true_positive as f64 / (true_positive + false_positive) as f64;
    let recall = true_positive as f64 / (true_positive + false_negative) as f64;
    let f1_score = 2.0 * (precision * recall) / (precision + recall);

    (precision, recall, f1_score)
}


