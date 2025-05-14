#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::command;
use rusqlite::{params, Connection};
use std::sync::Mutex;

#[derive(Clone)]
struct AppState {
    conn: Mutex<Connection>,
}

#[command]
fn agendar_banho_tosa(
    state: tauri::State<AppState>,
    nome: String,
    cpf: String,
    celular: String,
    nome_pet: String,
    especie: String,
    motivo: String,
    data: String,
    horario: String,
) -> Result<String, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS banho_tosa (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT NOT NULL,
            cpf TEXT NOT NULL,
            celular TEXT NOT NULL,
            nome_pet TEXT NOT NULL,
            especie TEXT NOT NULL,
            motivo TEXT NOT NULL,
            data TEXT NOT NULL,
            horario TEXT NOT NULL
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO banho_tosa (nome, cpf, celular, nome_pet, especie, motivo, data, horario) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![nome, cpf, celular, nome_pet, especie, motivo, data, horario],
    )
    .map_err(|e| e.to_string())?;

    Ok("Agendamento de Banho e Tosa salvo com sucesso".into())
}

#[command]
fn agendar_consulta(
    state: tauri::State<AppState>,
    nome: String,
    cpf: String,
    celular: String,
    nome_pet: String,
    especie: String,
    motivo: String,
    data: String,
    horario: String,
) -> Result<String, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS consultas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT NOT NULL,
            cpf TEXT NOT NULL,
            celular TEXT NOT NULL,
            nome_pet TEXT NOT NULL,
            especie TEXT NOT NULL,
            motivo TEXT NOT NULL,
            data TEXT NOT NULL,
            horario TEXT NOT NULL
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO consultas (nome, cpf, celular, nome_pet, especie, motivo, data, horario) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![nome, cpf, celular, nome_pet, especie, motivo, data, horario],
    )
    .map_err(|e| e.to_string())?;

    Ok("Agendamento de Consulta salvo com sucesso".into())
}

fn main() {
    let conn = Connection::open("projeto_rust.db").expect("Falha ao abrir banco de dados");

    tauri::Builder::default()
        .manage(AppState {
            conn: Mutex::new(conn),
        })
        .invoke_handler(tauri::generate_handler![agendar_banho_tosa, agendar_consulta])
        .run(tauri::generate_context!())
        .expect("Erro ao rodar o app Tauri");
}
