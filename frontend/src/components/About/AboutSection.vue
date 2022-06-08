<template>
  <div class="row">
    <AboutCardComponent child-class="bg-primary" title="About">
      Information in About Section Goes Here!
    </AboutCardComponent>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import AboutCardComponent from './AboutCardComponent.vue';

export default defineComponent({
  name: 'AboutSection',
  components: { AboutCardComponent },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
