use tauri::{command, Emitter, State, Window};

use crate::state::{MachineState, StateMachine};

#[command(async)]
pub fn start_counting(window: Window, target: u32, state_machine: State<'_, MachineState>) {
    println!(
        "start_counting] Commande reçue : démarrer le comptage avec cible {} ",
        target
    );
    state_machine.init_machine(window, target);
}

#[command(async)]
pub fn set_state(window: Window, state: String, state_machine: State<'_, MachineState>) {
    let mut new_state = StateMachine::Idle;
    if state == "play" {
        new_state = StateMachine::Play;
        send_state_update(window, state.to_string().as_str());
    } else if state == "pause" {
        new_state = StateMachine::Pause;
        send_state_update(window, state.to_string().as_str());
    }
    println!("Commande reçue : changement d'état à {}", state);
    state_machine.set_state(new_state);
}

#[command(async)]
pub fn set_debug(state_machine: State<'_, MachineState>) {
    println!("Commande reçue : changer le mode debug");

    state_machine.set_debug();
    print!("[set_debug] Mode debug actuel : {:?}", state_machine);
}

#[command]
pub fn get_state(state_machine: State<'_, MachineState>) -> StateMachine {
    println!("Commande reçue : récupérer l'état actuel.");
    state_machine.get_state()
}

// Émettre l'état ou le compteur depuis le backend
pub fn send_state_update(window: Window, state: &str) {
    window.emit("state-update", state).unwrap();
}

pub fn send_count_update(window: Window, count: u32) {
    window.emit("count-update", count).unwrap();
}
