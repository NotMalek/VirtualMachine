use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use virtual_machine::core::vm::VM;
use virtual_machine::core::assembler::Assembler;
use virtual_machine::core::state::DebugOptions;

// Shared state between requests
struct AppState {
    vm: Mutex<Option<VM>>,
}

#[derive(Debug, Serialize)]
struct VMStateResponse {
    stack: Vec<i64>,
    memory: std::collections::HashMap<String, i64>,
    program_counter: usize,
    output: Vec<String>,
    instructions: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LoadProgramRequest {
    code: String,
}

// Convert VM state to response format
impl From<&virtual_machine::core::state::VMState> for VMStateResponse {
    fn from(state: &virtual_machine::core::state::VMState) -> Self {
        VMStateResponse {
            stack: state.stack.clone(),
            memory: state.memory.clone(),
            program_counter: state.program_counter,
            output: vec![], // Will be filled separately
            instructions: state.instructions()
                .iter()
                .map(|i| i.to_string())
                .collect(),
        }
    }
}

// API endpoints
async fn load_program(
    data: web::Data<AppState>,
    program: web::Json<LoadProgramRequest>,
) -> Result<HttpResponse> {
    let mut assembler = Assembler::new();
    match assembler.assemble(&program.code) {
        Ok(instructions) => {
            let mut vm = VM::new(instructions);
            vm.set_debug_options(DebugOptions {
                show_instructions: true,
                show_stack: true,
                show_pc: false,
                show_memory: false,
            });

            let state = vm.get_state();
            let mut response = VMStateResponse::from(state);
            response.output = vm.take_output();

            // Store VM instance in app state
            let mut vm_state = data.vm.lock().unwrap();
            *vm_state = Some(vm);

            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Ok(HttpResponse::BadRequest().body(format!("Assembly error: {:?}", e))),
    }
}

async fn step(data: web::Data<AppState>) -> Result<HttpResponse> {
    println!("Step endpoint called");
    let mut vm_state = data.vm.lock().unwrap();

    if let Some(vm) = vm_state.as_mut() {
        println!("VM found, executing step");
        match vm.step() {
            Ok(true) => {  // Program still running
                let state = vm.get_state();
                let mut response = VMStateResponse::from(state);
                response.output = vm.take_output();
                println!("Step executed successfully");
                Ok(HttpResponse::Ok().json(response))
            }
            Ok(false) => {  // Program completed
                let state = vm.get_state();
                let mut response = VMStateResponse::from(state);
                response.output = vm.take_output();
                println!("Program completed");
                Ok(HttpResponse::Ok().json(response))
            }
            Err(e) => {
                println!("VM error: {:?}", e);
                Ok(HttpResponse::BadRequest().body(format!("VM error: {:?}", e)))
            }
        }
    } else {
        println!("No VM instance found");
        Ok(HttpResponse::BadRequest().body("No program loaded"))
    }
}

async fn reset(data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut vm_state = data.vm.lock().unwrap();
    *vm_state = None;
    Ok(HttpResponse::Ok().json("VM reset"))
}

async fn get_state(data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut vm_state = data.vm.lock().unwrap();

    if let Some(vm) = vm_state.as_mut() {
        let state = vm.get_state();
        let mut response = VMStateResponse::from(state);
        response.output = vm.take_output();
        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::BadRequest().body("No program loaded"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting VM server on http://127.0.0.1:3001");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()  // More permissive for development
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState {
                vm: Mutex::new(None),
            }))
            .service(
                web::scope("/api")
                    .route("/load", web::post().to(load_program))
                    .route("/step", web::post().to(step))
                    .route("/reset", web::post().to(reset))
                    .route("/state", web::get().to(get_state))
            )
    })
        .bind("127.0.0.1:3001")?
        .run()
        .await
}