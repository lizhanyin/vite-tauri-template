<script setup>

import { ref } from 'vue';
import { app, window, tauri } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event'
/* Set window top */

async function invoke(data, callback) {
  await tauri.invoke( 'event', {data: JSON.stringify(data)}).then((message) => callback(message));
}

async function setWindowTop(boolean) {
  let data = { command: 'set_window_top', value: boolean ? 'yes' : 'no' };
  await invoke(data, (message) => console.log(message));
}

let isListen = false;

const listening = async (boolean) => {
  let data = { command: 'init_process', value: boolean ? 'yes' : 'no' };
  let rst;
  await invoke(data, (message) => (rst = message, console.log(message))); // 调用rust任务
  if (!rst && !isListen){
    isListen = true;
    await listen('my-event', event => { // 监听事件
      console.log(event);
    });
  }
}

</script>

<template>
  <v-card
    prepend-icon="mdi-gesture-tap-button"
    title="Event"
    subtitle="Some Event Demo">

    <v-card-text class="d-flex align-center">
      <v-btn
        color="primary"
        class="ms-3"
        @click="setWindowTop(true)">
        top
      </v-btn>

      <v-btn
        color="primary"
        class="ms-3"
        @click="setWindowTop(false)">
        not top
      </v-btn>

      <v-btn
        color="primary"
        class="ms-3"
        @click="listening(true)">
        listen
      </v-btn>

      <v-btn
        color="primary"
        class="ms-3"
        @click="listening(false)">
        unlisten
      </v-btn>
    </v-card-text>


  </v-card>
</template>