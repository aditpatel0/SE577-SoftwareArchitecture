<template>
  <div>
    <div>
      <p>{{ user }}</p>
    </div>
    <q-btn label="Load Authenticated User" @click="loadAuthenticated" />
    <q-btn label="Load Unauthenticated User" @click="loadUnauthenticated" />
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import axios from 'axios';

const user = ref()

const loadAuthenticated = async () => {
    const res = await axios.get('http://localhost:3000/user?authenticated=true');
    const payload = JSON.stringify(res.data)
    user.value = payload
};

const loadUnauthenticated = async () => {
    const res = await axios.get('http://localhost:3000/user');
    const payload = JSON.stringify(res.data)
    user.value = payload
};

export default defineComponent({
  name: 'ProfilePageComponent',
  setup() {

    return {
      loadAuthenticated,
      loadUnauthenticated,
      user,
    };

  }
});

</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
