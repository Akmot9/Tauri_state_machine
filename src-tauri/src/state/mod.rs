use std::{sync::{Arc, Mutex}, thread, time::Duration};

use crate::commandes::{send_count_update, send_state_update};

#[derive(Clone, serde::Serialize, Debug)]
pub enum StateMachine {
    Idle,
    Play,
    Pause,
}

pub struct MachineState {
    state: Arc<Mutex<StateMachine>>,
}

impl Default for MachineState {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(StateMachine::Idle))
        }
        
    }
}

impl MachineState {
    pub fn set_state(&self, new_state: StateMachine) {
        let mut state = self.state.lock().unwrap();
        println!("Changement d'état : {:?}", new_state);
        *state = new_state;
    }

    pub fn get_state(&self) -> StateMachine {
        let state = self.state.lock().unwrap();
        println!("État actuel récupéré : {:?}", *state);
        state.clone()
    }

    pub fn start_counting(&self, window: tauri::Window, target: u32, debug: bool) {
        send_state_update(window.clone(), "play".into());
        let state_arc = Arc::clone(&self.state);
        println!("Démarrage du comptage. Cible : {} | Mode debug : {}", target, debug);
        thread::spawn(move || {
            for count in 1..=target {
                // Vérifier et attendre l'état "play" dans une section critique
                loop {
                    let state = {
                        let state = state_arc.lock().unwrap(); // verrouiller seulement dans cette section
                        state.clone()
                    };

                    if let StateMachine::Play = state {
                        break;
                    }

                    println!("La machine est en pause, en attente du mode play...");
                    thread::sleep(Duration::from_secs(1));
                }

                // Comptage
                println!("Comptage en cours... Valeur actuelle : {}", count);
                send_count_update(window.clone(), count);

                // Si le mode debug est activé, passer en pause après chaque incrément
                if debug {
                    let mut state = state_arc.lock().unwrap(); // verrouiller ici
                    println!("Mode debug activé, la machine passe en pause.");
                    *state = StateMachine::Pause;
                }

                // Simuler une attente entre les incréments de comptage
                thread::sleep(Duration::from_secs(1));
            }

            // Comptage terminé, la machine repasse à l'état idle
            let mut state = state_arc.lock().unwrap();
            println!("Comptage terminé, retour à l'état Idle.");
            *state = StateMachine::Idle;
        });
    }
}

