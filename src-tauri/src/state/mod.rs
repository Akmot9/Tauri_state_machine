use std::{fmt, sync::{Arc, Mutex}, thread, time::Duration};
use crate::commandes::{send_count_update, send_state_update};

#[derive(Clone, serde::Serialize, Debug)]
pub enum StateMachine {
    Idle,
    Play,
    Pause,
}

// Implémentation du trait Display pour StateMachine
impl fmt::Display for StateMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateMachine::Idle => write!(f, "idle"),
            StateMachine::Play => write!(f, "play"),
            StateMachine::Pause => write!(f, "pause"),
        }
    }
}

#[derive(Clone, serde::Serialize, Debug)]
pub struct MachineState {
    state: Arc<Mutex<StateMachine>>,
    debug: Arc<Mutex<bool>>, // Mode debug activé ou non
}

impl Default for MachineState {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(StateMachine::Idle)),
            debug: Arc::new(Mutex::new(false)),
        }
    }
}

impl MachineState {
    pub fn set_state(&self, new_state: StateMachine) {
        let mut state = self.state.lock().unwrap();
        println!("Changement d'état : {:?}", new_state);
        *state = new_state;
        println!("État actuel après changement : {:?}", *state);
    }

    pub fn get_state(&self) -> StateMachine {
        let state = self.state.lock().unwrap();
        println!("État actuel récupéré : {:?}", *state);
        state.clone()
    }

    pub fn init_machine(&self, window: tauri::Window, target: u32) {
        send_state_update(window.clone(), "idle".into());
        let state_arc = Arc::clone(&self.state);
        let debug_arc = Arc::clone(&self.debug); // Cloner l'Arc pour l'utiliser dans le thread
        println!("[start_counting] Démarrage du comptage avec cible {}", target);
    
        thread::spawn(move || {
            for count in 1..=target {
                // Vérifier et attendre l'état "play"
                loop {
                    let state = {
                        let state = state_arc.lock().unwrap(); // Verrouiller seulement ici pour récupérer l'état
                        state.clone()
                    };
                    println!("[loop] État de la machine à vérifier : {:?}", state);
                    send_state_update(window.clone(), state.to_string().as_str());
    
                    if let StateMachine::Play = state {
                        println!("[loop] État Play détecté, sortie de la boucle d'attente");
                        break;
                    }
    
                    println!("[loop] La machine n'est pas en mode play, en attente du mode play...");
                    thread::sleep(Duration::from_secs(1));
                }
    
                // Comptage
                println!("[start_counting] Comptage en cours... Valeur actuelle : {}", count);
                send_count_update(window.clone(), count);
    
                // Vérifier le mode debug avant de passer en pause
                let is_debug = *debug_arc.lock().unwrap();
                if is_debug {
                    let mut state = state_arc.lock().unwrap(); // Verrouiller seulement lors du changement d'état
                    println!("[start_counting] Mode debug activé, la machine passe en pause.");
                    *state = StateMachine::Pause;
                    println!("[start_counting] État actuel après passage en pause : {:?}", *state);
                }
    
                // Simuler une attente entre les incréments de comptage
                thread::sleep(Duration::from_secs(1));
            }
    
            // Comptage terminé, retour à l'état idle
            let mut state = state_arc.lock().unwrap();
            println!("[start_counting] Comptage terminé, retour à l'état Idle.");
            *state = StateMachine::Idle;
            println!("[start_counting] État actuel après retour à Idle : {:?}", *state);
            send_state_update(window, "idle".into());
    
            // Fin du thread
            println!("[start_counting] Le thread de comptage est terminé.");
        });
    
        // Ceci indique la fin de l'initialisation du thread (pas du thread lui-même)
        println!("[start_counting] Fin de l'initialisation du comptage avec cible. Cible : {}", target);
    }
    

    pub fn set_debug(&self) {
        let mut debug = self.debug.lock().unwrap(); // Verrouiller pour modifier la valeur
        *debug = !*debug;
        println!("Mode debug modifié. Nouveau mode debug : {}", *debug);
    }
}
