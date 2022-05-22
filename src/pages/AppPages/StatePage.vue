<template>
  <div class="q-pa-md">
    <q-banner class="bg-primary text-white">
      <h6>State Demo Page</h6>
    </q-banner>
    <div class="row">
      <div class="col-6 q-pa-sm">
        <StateCardComponent />
      </div>
      <div class="col-6 q-pa-sm">
        <StateCardComponent />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: 'StatePage',
};
</script>

<script setup lang="ts">
import StateCardComponent from 'src/components/StateCard.vue';
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
