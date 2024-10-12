use mysql::*;
use mysql::prelude::*;
use std::error::Error;


// Compara os dados das variaveis com as definições abaixo
#[derive(Debug, PartialEq, Eq)]
// Definindo tipos de dados em variaveis
struct User {
    id: i32,
    title: String,
    subject: String
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    // Configuração da conexão
    let url="mysql://root:admin%40%23%24@172.17.0.2:3306/tasks_db";
    /*
        na linha 27 teve um erro de conexão causada pela senha conter @#$ no, para contornar isso
        tive que codificar na url usando 'admin%40%23%24' no lugar de 'admin@#$', assim eu
        corrigi o erro sem alterar a senha no mysql diretamente.
    */
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    conn.query_drop(
        "
        CREATE TABLE IF NOT EXISTS tasks (
            id INT PRIMARY KEY AUTO_INCREMENT,
            name VARCHAR(50) NOT NULL,
            email VARCHAR(200) NOT NULL
        )
        "
    )?;

    println!("Tabela 'tasks' criada com sucesso!");

    Ok(())
}