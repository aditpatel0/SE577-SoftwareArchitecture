<template>
  <q-card class="my-card">
    <q-card-section>
      <div class="text-h6">Our Changing Planet - {{ title }}</div>
      <div class="text-subtitle2">by John Doe</div>
    </q-card-section>

    <q-separator />

    <q-card-actions vertical>
      <q-btn flat>Action 1</q-btn>
      <q-btn flat>Action 2</q-btn>
    </q-card-actions>
  </q-card>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'PropsChildComponent',
  props: { title: String },
});
</script>

Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
