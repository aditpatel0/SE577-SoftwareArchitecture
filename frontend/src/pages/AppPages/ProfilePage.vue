<template>
  <div>
    <div>
      <p>{{ user }}</p>
    </div>
    <q-btn label="Load Authenticated User" @click="loadAuthenticated" />
    <q-btn label="Load Unauthenticated User" @click="loadUnauthenticated" />

    <q-card class="my-card">
      <q-card-section>
         <br /> login: {{ user.login }}
         <br /> url: {{ user.url }}
         <br /> name: {{ user.name }}
         <br /> company: {{ user.company }}
         <br /> location: {{ user.location }}
         <br /> email: {{ user.email }}
         <br /> public_repos: {{ user.public_repos }}
         <br /> public_gists: {{ user.public_gists }}
         <br /> plan: {{ user.plan }}
      </q-card-section>
    </q-card>

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
