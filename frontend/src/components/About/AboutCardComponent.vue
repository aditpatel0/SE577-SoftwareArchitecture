<template>
  <div class="col-12">
    <q-card flat class="my-card text-white" :class="childClass">
      <q-card-section>
        <div class="text-h6">{{ title }}</div>
        <slot></slot>
      </q-card-section>
    </q-card>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'AboutCardComponent',
  props: {
    childClass: String,
    title: String,
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
