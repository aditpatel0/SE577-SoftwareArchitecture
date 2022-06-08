<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar>
        <q-toolbar-title> Welcome to Adit Patel's application </q-toolbar-title>
        <div>v 0.1</div>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <div id="app">
        <div id="nav">
          <router-link to="/">About Me</router-link> |
          <router-link to="/profile">Github Profile</router-link> |
          <router-link to="/gist">Gist</router-link> |
          <router-link to="/repositories">Github Repository</router-link>
        </div>
        <router-view />
      </div>
    </q-page-container>
  </q-layout>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c4c50;
}

#nav {
  padding: 30px;
}

#nav a {
  font-weight: bold;
  color: #503c2c;
}

#nav a.router-link-exact-active {
  color: #21007a;
}
</style>
