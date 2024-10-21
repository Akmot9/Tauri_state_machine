<template>
  <div id="app">
    <h1>Machine à état - Mode Compte</h1>

    <label>
      <input type="checkbox" v-model="isDebugMode" @change="toggleDebugMode" />
      Activer le mode Debug
    </label>

    <p>État actuel : {{ currentState }}</p>
    <p>Compteur actuel : {{ currentCount }}</p>

    <input type="number" v-model="targetCount" placeholder="Valeur de comptage" />

    <!-- Bouton pour basculer entre Play et Pause -->
    <button @click="togglePlayPause">
      {{ currentState === 'play' ? 'Pause' : 'Play' }}
    </button>
    <button @click="startCounting">reset</button>

    
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export default defineComponent({
  data() {
    return {
      currentState: 'idle', // État initial
      currentCount: 0, // Compteur actuel
      targetCount: 10, // Valeur cible pour le comptage
      isDebugMode: false // Mode Debug activé/désactivé
    };
  },
  methods: {
    startCounting() {
      // Appel à Tauri pour démarrer le comptage
      invoke('start_counting', { target: this.targetCount })
        .then((message) => {
          console.log('Comptage démarré:', message);
        })
        .catch((error) => {
          console.error('Erreur lors du démarrage du comptage:', error);
        });
    },
    togglePlayPause() {
      console.log('Clique sur le bouton Play/Pause', this.currentState);
      if (this.currentState === 'play') {
        this.currentState = 'pause';
        invoke('set_state', { state: 'pause' })
          .then((message) => {
            console.log('La machine est en pause:', message);
          })
          .catch((error) => {
            console.error('Erreur lors de la mise en pause:', error);
          });
      } else if (this.currentState === 'pause') {
        this.currentState = 'play';
        invoke('set_state', { state: 'play' })
          .then((message) => {
            console.log('La machine est en mode play:', message);
          })
          .catch((error) => {
            console.error('Erreur lors du passage en mode play:', error);
          });
      } else if (this.currentState === 'idle') {
        this.currentState = 'play';
        invoke('set_state', { state: 'play' })
          .then((message) => {
            console.log('La machine est en mode play:', message);
          })
          .catch((error) => {
            console.error('Erreur lors du passage en mode play:', error);
          });
      } else {
        console.error('Etat invalide');
      }
    },
    toggleDebugMode() {
      // Inverser le mode debug dans le backend
      console.log('Toggle du mode Debug, nouveau mode :', this.isDebugMode);
      invoke('set_debug')
        .then((message) => {
          console.log('Mode debug modifié :', message);
        })
        .catch((error) => {
          console.error('Erreur lors du changement de mode debug:', error);
        });
    },
    // Méthode pour écouter les événements Tauri
    listenForUpdates() {
      // Écouter les mises à jour de l'état
      listen('state-update', (event) => {
        this.currentState = event.payload;
        console.log('État mis à jour depuis le backend:', this.currentState);
      });

      // Écouter les mises à jour du compteur
      listen('count-update', (event) => {
        this.currentCount = event.payload;
        console.log('Compteur mis à jour depuis le backend:', this.currentCount);
      });
    }
  },
  mounted() {
    // Écouter les événements depuis le backend
    this.listenForUpdates();
    this.startCounting();
  }
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  text-align: center;
  margin-top: 60px;
}

button {
  margin: 10px;
  padding: 10px 20px;
  font-size: 16px;
}
</style>
