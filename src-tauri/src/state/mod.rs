use std::{sync::{Arc, Mutex}, thread, time::Duration};



use crate::commandes::{send_count_update, send_state_update};

#[derive(Clone, serde::Serialize, Debug)]
pub enum StateMachine {
    Idle,
    Play,
    Pause,
}

#[derive(Clone, serde::Serialize, Debug)]
pub struct MachineState {
    state: Arc<Mutex<StateMachine>>,
    debug: Arc<Mutex<bool>>
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

    pub fn init_machine(&self, window: tauri::Window, target: u32, debug: bool) {
        send_state_update(window.clone(), "idle".into());
        let state_arc = Arc::clone(&self.state);
        println!("[start_counting] Démarrage du comptage avec cible {} | Mode debug : {}", target, debug);
        thread::spawn(move || {
            for count in 1..=target {
                // Vérifier et attendre l'état "play" dans une section critique
                loop {
                    let state = {
                        let state = state_arc.lock().unwrap(); // verrouiller seulement dans cette section
                        state.clone()
                    };
                    println!("[loop] État de la machine à vérifier : {:?}", state);

                    if let StateMachine::Play = state {
                        println!("[loop] État Play détecté, sortie de la boucle d'attente");

                        break;
                    }

                    println!("[loop] La machine est pas en mode play, en attente du mode play...");
                    thread::sleep(Duration::from_secs(1));
                }

                // Comptage
                println!("[start_counting] Comptage en cours... Valeur actuelle : {}", count);
                send_count_update(window.clone(), count);

                // Si le mode debug est activé, passer en pause après chaque incrément
                if debug {
                    let mut state = state_arc.lock().unwrap(); // verrouiller ici
                    println!("[start_counting] Mode debug activé, la machine passe en pause.");
                    *state = StateMachine::Pause;
                    println!("[start_counting] État actuel après passage en pause : {:?}", *state);

                }

                // Simuler une attente entre les incréments de comptage
                thread::sleep(Duration::from_secs(1));
            }

            // Comptage terminé, la machine repasse à l'état idle
            let mut state = state_arc.lock().unwrap();
            println!("[start_counting] Comptage terminé, retour à l'état Idle.");
            *state = StateMachine::Idle;
            println!("[start_counting] État actuel après retour à Idle : {:?}", *state);
            send_state_update(window.clone(), "idle".into());


        });
    }

    pub fn set_debug(&self) {
        let mut debug = self.debug.lock().unwrap();
        *debug = !*debug;
    }
}

