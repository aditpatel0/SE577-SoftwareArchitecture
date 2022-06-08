<template>
  <div class="q-pa-md row items-start">
    <q-card flat class="my-card bg-primary text-white">
      <q-card-section>
        <div class="text-h6">About</div>
        {{ lorem }}
      </q-card-section>
    </q-card>
    <q-card flat class="my-card bg-secondary text-white">
      <q-card-section>
        <div class="text-h6">Background</div>
        {{ lorem }}
      </q-card-section>
    </q-card>
    <q-card flat class="my-card bg-accent text-white">
      <q-card-section>
        <div class="text-h6">Course</div>
        {{ lorem }}
      </q-card-section>
    </q-card>
    <q-card flat class="my-card bg-info text-white">
      <q-card-section>
        <div class="text-h6">Publications</div>
        {{ lorem }}
      </q-card-section>
    </q-card>
    <q-card flat class="my-card bg-warning text-white">
      <q-card-section>
        <div class="text-h6">Photos</div>
        {{ lorem }}
      </q-card-section>
    </q-card>
  </div>
  <!-- About, Background, Course, Publications, Photo-->
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'AboutPageComponent',
  setup() {
    return {
      lorem:
        'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.',
    };
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
