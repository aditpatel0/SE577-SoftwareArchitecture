<template>
  <div>
    <p>{{ gist }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import axios from 'axios';

const gist = ref()

onMounted(async () => {
  const res = await axios.get('http://localhost:3000/gist');
  gist.value = res.data.files['Python Hello'].content;
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>

